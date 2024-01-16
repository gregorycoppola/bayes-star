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

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum InferenceNodeType {
    PropositionHash(u64),
    ConjunctHash(u64),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct InferenceNode {
    node_type: InferenceNodeType,
    debug_string:Option<String>,
}

impl InferenceNode {
    // Constructor for an InferenceNode with a Proposition
    pub fn from_proposition(proposition: &Proposition) -> InferenceNode {
        let mut hasher = DefaultHasher::new();
        proposition.hash(&mut hasher);
        let hash = hasher.finish();

        InferenceNode {
            node_type: InferenceNodeType::PropositionHash(hash),
            debug_string: Some(serialize_record(&proposition).unwrap()),
        }
    }

    // Constructor for an InferenceNode with a Conjunct
    pub fn from_conjunct(group: &PropositionGroup) -> InferenceNode {
        let mut hasher = DefaultHasher::new();
        group.hash(&mut hasher);
        let hash = hasher.finish();

        InferenceNode {
            node_type: InferenceNodeType::ConjunctHash(hash),
            debug_string: Some(serialize_record(&group).unwrap()),
        }
    }
}

pub struct HashMapBeliefTable {
    pi_values: HashMap<(InferenceNode, usize), f64>,
    lambda_values: HashMap<(InferenceNode, usize), f64>,
    pi_messages: HashMap<(InferenceNode, InferenceNode, usize), f64>,
    lambda_messages: HashMap<(InferenceNode, InferenceNode, usize), f64>,
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
    pub fn get_pi_value(&self, node: &InferenceNode, outcome: usize) -> Option<f64> {
        let key = (node.clone(), outcome);
        self.pi_values.get(&key).cloned()
    }

    // Setter for pi values
    pub fn set_pi_value(&mut self, node: &InferenceNode, outcome: usize, value: f64) {
        let key = (node.clone(), outcome);
        self.pi_values.insert(key, value);
    }

    // Getter for lambda values
    pub fn get_lambda_value(&self, node: &InferenceNode, outcome: usize) -> Option<f64> {
        let key = (node.clone(), outcome);
        self.lambda_values.get(&key).cloned()
    }

    // Setter for lambda values
    pub fn set_lambda_value(&mut self, node: &InferenceNode, outcome: usize, value: f64) {
        let key = (node.clone(), outcome);
        self.lambda_values.insert(key, value);
    }

    // Getter for pi messages
    pub fn get_pi_message(
        &self,
        from: &InferenceNode,
        to: &InferenceNode,
        outcome: usize,
    ) -> Option<f64> {
        let key = (from.clone(), to.clone(), outcome);
        self.pi_messages.get(&key).cloned()
    }

    // Setter for pi messages
    pub fn set_pi_message(
        &mut self,
        from: &InferenceNode,
        to: &InferenceNode,
        outcome: usize,
        value: f64,
    ) {
        let key = (from.clone(), to.clone(), outcome);
        self.pi_messages.insert(key, value);
    }

    // Getter for lambda messages
    pub fn get_lambda_message(
        &self,
        from: &InferenceNode,
        to: &InferenceNode,
        outcome: usize,
    ) -> Option<f64> {
        let key = (from.clone(), to.clone(), outcome);
        self.lambda_messages.get(&key).cloned()
    }

    // Setter for lambda messages
    pub fn set_lambda_message(
        &mut self,
        from: &InferenceNode,
        to: &InferenceNode,
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
