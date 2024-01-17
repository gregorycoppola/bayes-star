use super::{
    graph::PropositionGraph,
    table::{HashMapBeliefTable, InferenceResult, PropositionNode},
};
use crate::{
    common::{interface::PropositionDB, model::InferenceModel},
    inference::table::{HashMapInferenceResult, GenericNodeType},
    model::{
        objects::{Predicate, PredicateGroup, Proposition, PropositionGroup, EXISTENCE_FUNCTION},
        weights::CLASS_LABELS,
    }, print_red, print_yellow, print_green,
};
use redis::Connection;
use std::{borrow::Borrow, collections::HashMap, error::Error, rc::Rc};

struct Inferencer {
    model: Rc<InferenceModel>,
    proposition_graph: Rc<PropositionGraph>,
    pub data: HashMapBeliefTable,
}

fn inference_proposition_probability(
    proposition_db: &dyn PropositionDB,
    proposition: &Proposition,
) -> Result<f64, Box<dyn Error>> {
    Ok(proposition_db
        .get_proposition_probability(proposition)
        .unwrap()
        .unwrap())
}

fn inference_conjoined_probability(
    proposition_db: &dyn PropositionDB,
    group: &PropositionGroup,
) -> Result<f64, Box<dyn Error>> {
    todo!()
}

impl Inferencer {
    // Initialize new Storage with a Redis connection
    pub fn new_mutable(
        model: Rc<InferenceModel>,
        proposition_graph: Rc<PropositionGraph>,
    ) -> Result<Box<Self>, redis::RedisError> {
        Ok(Box::new(Inferencer {
            model,
            proposition_graph,
            data: HashMapBeliefTable::new(),
        }))
    }

    pub fn initialize(&mut self, proposition: &Proposition) -> Result<(), Box<dyn Error>> {
        print_red!("initialize: proposition {:?}", proposition.hash_string());
        // self.initialize_pi()?;
        self.initialize_lambda()?;
        self.initialize_pi_roots()?;
        Ok(())
    }

    pub fn initialize_lambda(&mut self) -> Result<(), Box<dyn Error>> {
        print_red!("initialize_lambda: proposition");
        for node in &self.proposition_graph.all_nodes {
            print_red!("initializing: {}", node.debug_string());
            for outcome in CLASS_LABELS {
                self.data.set_lambda_value(node, outcome, 1f64);
            }
            for parent in &self.proposition_graph.get_all_backward(node) {
                print_red!(
                    "initializing lambda link from {} to {}",
                    node.debug_string(),
                    parent.debug_string()
                );
                for outcome in CLASS_LABELS {
                    self.data.set_lambda_message(node, parent, outcome, 1f64);
                }
            }
        }
        Ok(())
    }

    pub fn initialize_pi_roots(&mut self) -> Result<(), Box<dyn Error>> {
        let roots = self.proposition_graph.roots.clone();
        for root in &roots {
            assert_eq!(root.predicate.function, EXISTENCE_FUNCTION.to_string());
            self.data.set_pi_value(&PropositionNode::from_single(&root), 1, 1.0f64);
            self.data.set_pi_value(&PropositionNode::from_single(&root), 0, 0.0f64);
        }

        for root in &roots {
            let node = PropositionNode::from_single(root);
            self.pi_visit_node(&node)?;
        }
        print_yellow!("{:?}", &roots);
        Ok(())
    }

    pub fn pi_visit_node(&mut self, from_node:&PropositionNode) -> Result<(), Box<dyn Error>> {
        // Part 1: For each value of z, compute pi_X(z)
        let forward_groups = self.proposition_graph.get_all_forward(from_node);
        for (this_index, to_node) in forward_groups.iter().enumerate() {
            for class_label in &CLASS_LABELS {
                let mut lambda_part = 1f64;
                for (other_index, other_node) in forward_groups.iter().enumerate() {
                    if other_index != this_index {
                        let this_lambda = self.data.get_lambda_value(&other_node, *class_label).unwrap();
                        lambda_part *= this_lambda;
                    }
                }
                let pi_part = self.data.get_pi_value(&to_node, *class_label).unwrap();
                let message = pi_part * lambda_part;
                self.data.set_pi_message(&from_node, &to_node, *class_label, message);
            }
        }
        // Part 2: For children not in evidence, recursive into.
        todo!()
    }

    pub fn pi_compute_generic(&mut self, node:&PropositionNode) -> Result<(), Box<dyn Error>> {
        match &node.node {
            GenericNodeType::Single(proposition) => {
                self.pi_compute_single(node)?;
            }
            GenericNodeType::Group(group) => {
                self.pi_compute_group(node)?;
            }
        }
        todo!()
    }

    pub fn pi_compute_single(&mut self, node:&PropositionNode) -> Result<(), Box<dyn Error>> {
        let backlinks = self.proposition_graph.get_all_backward(node);
        todo!()
    }

    pub fn pi_compute_group(&mut self, from_node:&PropositionNode) -> Result<(), Box<dyn Error>> {
        let backlinks = self.proposition_graph.get_all_backward(from_node);
        let mut product = 1f64;
        for (index, to_node) in backlinks.iter().enumerate() {
            let pi_x_z = self.data.get_lambda_message(from_node, to_node, 1).unwrap();
            product *= pi_x_z;
        }
        self.data.set_pi_value(from_node, 1, product);
        todo!()
    }
}

fn each_combination(propositions: &Vec<PropositionNode>) -> Vec<HashMap<PropositionNode, bool>> {
    let n = propositions.len();
    let mut all_combinations = Vec::new();
    for i in 0..(1 << n) {
        let mut current_combination = HashMap::new();
        for j in 0..n {
            let prop = &propositions[j];
            let state = i & (1 << j) != 0;
            current_combination.insert(prop.clone(), state);
        }
        all_combinations.push(current_combination);
    }
    all_combinations
}


// Note: GraphicalModel contains PropositionDB, which contains the "evidence".
pub fn inference_compute_marginals(
    model: Rc<InferenceModel>,
    target: &Proposition,
) -> Result<Rc<dyn InferenceResult>, Box<dyn Error>> {
    let proposition_graph = PropositionGraph::new_shared(model.graph.clone(), target)?;
    let mut inferencer = Inferencer::new_mutable(model.clone(), proposition_graph.clone())?;
    inferencer.initialize(target)?;
    inferencer.data.print_debug();
    HashMapInferenceResult::new_shared(inferencer.data)
}
