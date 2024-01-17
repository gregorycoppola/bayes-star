use super::{
    graph::PropositionGraph,
    table::{HashMapBeliefTable, InferenceResult, PropositionNode},
};
use crate::{
    common::{interface::PropositionDB, model::InferenceModel},
    inference::table::HashMapInferenceResult,
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
            self.data.set_pi_value(&PropositionNode::from_proposition(&root), 1, 1.0f64);
            self.data.set_pi_value(&PropositionNode::from_proposition(&root), 0, 0.0f64);
        }

        for root in &roots {
            self.send_pi_from_single(root)?;
        }
        print_yellow!("{:?}", &roots);
        Ok(())
    }

    pub fn send_pi_from_group(&mut self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub fn send_pi_from_single(&mut self, proposition:&Proposition) -> Result<(), Box<dyn Error>> {
        let forward_groups = self.proposition_graph.get_single_forward(proposition);
        for (this_index, this_value) in forward_groups.iter().enumerate() {
            for class_label in &CLASS_LABELS {
                let mut lambda_part = 1f64;
                for (other_index, other_value) in forward_groups.iter().enumerate() {
                    if other_index != this_index {
                        let node = PropositionNode::from_group(other_value);
                        let this_lambda = self.data.get_lambda_value(&node, *class_label).unwrap();
                        lambda_part *= this_lambda;
                    }
                }
            }
        }
        todo!()
    }
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
