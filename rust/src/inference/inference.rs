use super::{
    graph::{PropositionFactor, PropositionGraph},
    table::{HashMapBeliefTable, PropositionNode},
};
use crate::{
    common::{
        interface::PropositionDB,
        model::{FactorContext, InferenceModel},
        proposition_db,
    },
    inference::table::GenericNodeType,
    model::{
        objects::{Predicate, PredicateGroup, Proposition, PropositionGroup, EXISTENCE_FUNCTION},
        weights::CLASS_LABELS,
    },
    print_blue, print_green, print_red, print_yellow,
};
use redis::Connection;
use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    rc::Rc,
};

pub struct Inferencer {
    pub model: Rc<InferenceModel>,
    pub fact_memory: Rc<dyn PropositionDB>,
    pub proposition_graph: Rc<PropositionGraph>,
    pub data: HashMapBeliefTable,
    pub bfs_order: Vec<PropositionNode>,
}

fn reverse_prune_duplicates(raw_order: &Vec<(i32, PropositionNode)>) -> Vec<PropositionNode> {
    let mut seen = HashSet::new();
    let mut result = vec![];
    for (depth, node) in raw_order.iter().rev() {
        if !seen.contains(node) {
            result.push(node.clone());
        }
        seen.insert(node);
    }
    result.reverse();
    result
}

fn create_bfs_order(proposition_graph: &PropositionGraph) -> Vec<PropositionNode> {
    let mut queue = VecDeque::new();
    let mut buffer = vec![];
    for root in &proposition_graph.roots {
        queue.push_back((0, PropositionNode::from_single(&root)));
    }

    print_yellow!("create_bfs_order initial: queue {:?}", &queue);

    while let Some((depth, node)) = queue.pop_front() {
        buffer.push((depth, node.clone()));
        let forward = proposition_graph.get_all_forward(&node);
        for child in &forward {
            queue.push_back((depth + 1, child.clone()));
        }

        print_yellow!("create_bfs_order initial: queue {:?}", &queue);
        print_yellow!("create_bfs_order initial: buffer {:?}", &buffer);
    }

    let result = reverse_prune_duplicates(&buffer);
    print_yellow!("create_bfs_order result: {:?}", &result);
    result
}

impl Inferencer {
    // Initialize new Storage with a Redis connection
    pub fn new_mutable(
        model: Rc<InferenceModel>,
        proposition_graph: Rc<PropositionGraph>,
        fact_memory: Rc<dyn PropositionDB>,
    ) -> Result<Box<Self>, redis::RedisError> {
        let bfs_order = create_bfs_order(&proposition_graph);
        Ok(Box::new(Inferencer {
            model,
            fact_memory,
            proposition_graph,
            data: HashMapBeliefTable::new(),
            bfs_order,
        }))
    }

    pub fn initialize(&mut self, proposition: &Proposition) -> Result<(), Box<dyn Error>> {
        print_red!("initialize: proposition {:?}", proposition.hash_string());
        // self.initialize_pi()?;
        self.initialize_lambda()?;
        self.do_pi_traversal()?;
        self.do_lambda_traversal()?;
        self.update_marginals()?;
        Ok(())
    }

    pub fn update_marginals(&mut self) -> Result<(), Box<dyn Error>> {
        print_red!("update_marginals over {:?}", &self.bfs_order);
        for node in &self.bfs_order {
            let pi0 = self.data.get_pi_value(node, 0).unwrap();
            let pi1 = self.data.get_pi_value(node, 1).unwrap();
            let lambda0 = self.data.get_lambda_value(node, 0).unwrap();
            let lambda1 = self.data.get_lambda_value(node, 1).unwrap();
            let potential0 = pi0 * lambda0;
            let potential1 = pi1 * lambda1;
            let norm = potential0 + potential1;
            let probability0 = potential0 / norm;
            let probability1 = potential1 / norm;
            print_red!("node {:?} p0 {} p1 {}", node, probability0, probability1);
        }
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

    pub fn is_root(&self, node: &PropositionNode) -> bool {
        if node.is_single() {
            let as_single = node.extract_single();
            let is_root = self.proposition_graph.roots.contains(&as_single);
            is_root
        } else {
            false
        }
    }

    pub fn is_leaf(&self, node: &PropositionNode) -> bool {
        if node.is_single() {
            let as_single = node.extract_single();
            let forward_links = self.proposition_graph.single_forward.get(&as_single).unwrap();
            forward_links.is_empty()
        } else {
            false
        }
    }

    pub fn is_observed(&self, node: &PropositionNode) -> Result<bool, Box<dyn Error>> {
        if node.is_single() {
            let as_single = node.extract_single();
            let has_evidence = self
                .fact_memory
                .get_proposition_probability(&as_single)?
                .is_some();
            print_green!(
                "is_observed? node {:?}, has_evidence {}",
                &as_single,
                has_evidence
            );
            Ok(has_evidence)
        } else {
            Ok(false)
        }
    }

    // TODO: move this out of the class
    pub fn build_factor_context_for_assignment(
        &self,
        premises: &Vec<PropositionGroup>,
        premise_assignment: &HashMap<PropositionNode, bool>,
        conclusion: &Proposition,
    ) -> FactorContext {
        let mut probabilities = vec![];
        let mut factors = vec![];
        for proposition_group in premises {
            let node = PropositionNode::from_group(proposition_group);
            let assignment = *premise_assignment.get(&node).unwrap();
            if assignment {
                probabilities.push(1f64);
            } else {
                probabilities.push(0f64);
            }
            let inference = self
                .proposition_graph
                .get_inference_used(proposition_group, conclusion);
            let factor = PropositionFactor {
                premise: proposition_group.clone(),
                conclusion: conclusion.clone(),
                inference,
            };
            factors.push(factor);
        }
        let context = FactorContext {
            factor: factors,
            probabilities,
        };
        context
    }
}

// Return 1 HashMap for each of the 2^N ways to assign each of the N memebers of `propositions` to either true or false.
pub fn compute_each_combination(
    propositions: &Vec<PropositionNode>,
) -> Vec<HashMap<PropositionNode, bool>> {
    print_yellow!("compute_each_combination: propositions={:?}", &propositions);
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
    fact_memory: Rc<dyn PropositionDB>,
    target: &Proposition,
) -> Result<(), Box<dyn Error>> {
    let proposition_graph = PropositionGraph::new_shared(model.graph.clone(), target)?;
    proposition_graph.visualize();
    let mut inferencer = Inferencer::new_mutable(model.clone(), proposition_graph.clone(), fact_memory)?;
    inferencer.initialize(target)?;
    inferencer.data.print_debug();
    Ok(())
}

pub fn groups_from_backlinks(backlinks: &Vec<PropositionNode>) -> Vec<PropositionGroup> {
    let mut result = vec![];
    for backlink in backlinks {
        let group = backlink.extract_group();
        result.push(group);
    }
    result
}
