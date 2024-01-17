use crate::{
    common::{interface::PropositionDB, graph::serialize_record},
    model::{
        objects::{Predicate, PredicateGroup, Proposition, PropositionGroup},
        weights::CLASS_LABELS,
    },
};
use redis::Connection;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, rc::Rc};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum GenericNodeType {
    Single(Proposition),
    Group(PropositionGroup),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct PropositionNode {
    node: GenericNodeType,
    debug_string: String,
}

impl PropositionNode {
    // Constructor for an InferenceNode with a Proposition
    pub fn from_proposition(proposition: &Proposition) -> PropositionNode {
        PropositionNode {
            node: GenericNodeType::Single(proposition.clone()),
            debug_string: proposition.hash_string(),
        }
    }

    // Constructor for an InferenceNode with a Conjunct
    pub fn from_group(group: &PropositionGroup) -> PropositionNode {
        PropositionNode {
            node: GenericNodeType::Group(group.clone()),
            debug_string: group.hash_string(),
        }
    }

    pub fn debug_string(&self) -> String {
        match &self.node {
            GenericNodeType::Single(proposition) => proposition.debug_string(),
            GenericNodeType::Group(group) => group.debug_string(),
        }
    }
}

#[derive(Debug, Clone)]

pub struct HashMapBeliefTable {
    pi_values: HashMap<(PropositionNode, usize), f64>,
    lambda_values: HashMap<(PropositionNode, usize), f64>,
    pi_messages: HashMap<(PropositionNode, PropositionNode, usize), f64>,
    lambda_messages: HashMap<(PropositionNode, PropositionNode, usize), f64>,
}

fn print_sorted_map(map: &HashMap<(PropositionNode, usize), f64>) {
    let mut map_entries: Vec<_> = map.iter().collect();
    
    // Sorting by InferenceNode.debug_string and then by usize
    map_entries.sort_by(|a, b| {
        let ((node_a, index_a), _) = a;
        let ((node_b, index_b), _) = b;

        match node_a.debug_string.cmp(&node_b.debug_string) {
            std::cmp::Ordering::Equal => index_a.cmp(index_b),
            other => other,
        }
    });

    // Printing in sorted order
    for ((node, index), value) in map_entries {
        info!("{} ({}): {}", node.debug_string, index, value);
    }
}

fn print_sorted_messages(map: &HashMap<(PropositionNode, PropositionNode, usize), f64>) {
    let mut map_entries: Vec<_> = map.iter().collect();

    // Sorting by the first InferenceNode.debug_string, then the second, and then by usize
    map_entries.sort_by(|a, b| {
        let ((node_a1, node_a2, index_a), _) = a;
        let ((node_b1, node_b2, index_b), _) = b;

        match node_a1.debug_string.cmp(&node_b1.debug_string) {
            std::cmp::Ordering::Equal => match node_a2.debug_string.cmp(&node_b2.debug_string) {
                std::cmp::Ordering::Equal => index_a.cmp(index_b),
                other => other,
            },
            other => other,
        }
    });

    // Printing in sorted order
    for ((node1, node2, index), value) in map_entries {
        info!("{} - {} ({}): {}", node1.debug_string, node2.debug_string, index, value);
    }
}


impl HashMapBeliefTable {
    pub fn print_debug(&self) {
        info!("pi_values:");
        print_sorted_map(&self.pi_values);
        info!("lambda_values:");
        print_sorted_map(&self.lambda_values);
        info!("pi_messages:");
        print_sorted_messages(&self.pi_messages);
        info!("lambda_messages:");
        print_sorted_messages(&self.lambda_messages);
    }
}

impl HashMapBeliefTable {
    // Constructor to create a new instance
    pub fn new() -> Self {
        HashMapBeliefTable {
            pi_values: HashMap::new(),
            lambda_values: HashMap::new(),
            pi_messages: HashMap::new(),
            lambda_messages: HashMap::new(),
        }
    }

    // Getter for pi values
    pub fn get_pi_value(&self, node: &PropositionNode, outcome: usize) -> Option<f64> {
        let key = (node.clone(), outcome);
        self.pi_values.get(&key).cloned()
    }

    // Setter for pi values
    pub fn set_pi_value(&mut self, node: &PropositionNode, outcome: usize, value: f64) {
        let key = (node.clone(), outcome);
        self.pi_values.insert(key, value);
    }

    // Getter for lambda values
    pub fn get_lambda_value(&self, node: &PropositionNode, outcome: usize) -> Option<f64> {
        let key = (node.clone(), outcome);
        self.lambda_values.get(&key).cloned()
    }

    // Setter for lambda values
    pub fn set_lambda_value(&mut self, node: &PropositionNode, outcome: usize, value: f64) {
        let key = (node.clone(), outcome);
        self.lambda_values.insert(key, value);
    }

    // Getter for pi messages
    pub fn get_pi_message(
        &self,
        from: &PropositionNode,
        to: &PropositionNode,
        outcome: usize,
    ) -> Option<f64> {
        let key = (from.clone(), to.clone(), outcome);
        self.pi_messages.get(&key).cloned()
    }

    // Setter for pi messages
    pub fn set_pi_message(
        &mut self,
        from: &PropositionNode,
        to: &PropositionNode,
        outcome: usize,
        value: f64,
    ) {
        let key = (from.clone(), to.clone(), outcome);
        self.pi_messages.insert(key, value);
    }

    // Getter for lambda messages
    pub fn get_lambda_message(
        &self,
        from: &PropositionNode,
        to: &PropositionNode,
        outcome: usize,
    ) -> Option<f64> {
        let key = (from.clone(), to.clone(), outcome);
        self.lambda_messages.get(&key).cloned()
    }

    // Setter for lambda messages
    pub fn set_lambda_message(
        &mut self,
        from: &PropositionNode,
        to: &PropositionNode,
        outcome: usize,
        value: f64,
    ) {
        let key = (from.clone(), to.clone(), outcome);
        self.lambda_messages.insert(key, value);
    }
}

pub struct HashMapInferenceResult {
    underlying: HashMapBeliefTable,
}

impl HashMapInferenceResult {
    pub fn new_shared(
        underlying: HashMapBeliefTable,
    ) -> Result<Rc<dyn InferenceResult>, Box<dyn Error>> {
        Ok(Rc::new(HashMapInferenceResult { underlying }))
    }
}

impl InferenceResult for HashMapInferenceResult {
    fn get_proposition_probability(&self, proposition: &Predicate) -> Result<f64, Box<dyn Error>> {
        todo!()
    }
}

pub trait InferenceResult {
    fn get_proposition_probability(&self, proposition: &Predicate) -> Result<f64, Box<dyn Error>>;
}
