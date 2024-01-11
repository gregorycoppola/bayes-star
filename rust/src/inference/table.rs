use std::{collections::HashMap, error::Error};
use redis::Connection;
use serde::{Serialize, Deserialize};
use crate::{model::{objects::{Proposition, Conjunct}, weights::CLASS_LABELS}, common::interface::FactDB};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum InferenceNodeType {
    Proposition(Proposition),
    Conjunct(Conjunct),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct InferenceNode {
    node_type: InferenceNodeType,
    // other fields for InferenceNode
}

impl InferenceNode {
    // Constructor for an InferenceNode with a Proposition
    pub fn from_proposition(proposition: Proposition) -> InferenceNode {
        InferenceNode {
            node_type: InferenceNodeType::Proposition(proposition),
        }
    }

    // Constructor for an InferenceNode with a Conjunct
    pub fn from_conjunct(conjunct: Conjunct) -> InferenceNode {
        InferenceNode {
            node_type: InferenceNodeType::Conjunct(conjunct),
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
        let key = (*node, outcome);
        self.pi_values.get(&key).cloned()
    }

    // Setter for pi values
    pub fn set_pi_value(&mut self, node: &InferenceNode, outcome: usize, value: f64) {
        let key = (*node, outcome);
        self.pi_values.insert(key, value);
    }

    // Getter for lambda values
    pub fn get_lambda_value(&self, node: &InferenceNode, outcome: usize) -> Option<f64> {
        let key = (*node, outcome);
        self.lambda_values.get(&key).cloned()
    }

    // Setter for lambda values
    pub fn set_lambda_value(&mut self, node: &InferenceNode, outcome: usize, value: f64) {
        let key = (*node, outcome);
        self.lambda_values.insert(key, value);
    }

    // Getter for pi messages
    pub fn get_pi_message(
        &self,
        from: &InferenceNode,
        to: &InferenceNode,
        outcome: usize,
    ) -> Option<f64> {
        let key = (*from, *to, outcome);
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
        let key = (*from, *to, outcome);
        self.pi_messages.insert(key, value);
    }

    // Getter for lambda messages
    pub fn get_lambda_message(
        &self,
        from: &InferenceNode,
        to: &InferenceNode,
        outcome: usize,
    ) -> Option<f64> {
        let key = (*from, *to, outcome);
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
        let key = (*from, *to, outcome);
        self.lambda_messages.insert(key, value);
    }
}

struct HashMapInferenceResult {
    underlying: HashMapBeliefTable,
}

impl InferenceResult for HashMapInferenceResult {
    fn get_proposition_probability(&self, proposition:&Proposition) -> Result<f64, Box<dyn Error>> {
        todo!()
    }
}

pub trait InferenceResult {
    fn get_proposition_probability(&self, proposition:&Proposition) -> Result<f64, Box<dyn Error>>;
}