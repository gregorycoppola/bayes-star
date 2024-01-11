use std::{collections::HashMap, error::Error};

use redis::Connection;

use crate::{model::{objects::Proposition, weights::CLASS_LABELS}, common::interface::FactDB};

pub struct HashMapBeliefTable {
    pi_values: HashMap<(String, usize), f64>,
    lambda_values: HashMap<(String, usize), f64>,
    pi_messages: HashMap<(String, String, usize), f64>,
    lambda_messages: HashMap<(String, String, usize), f64>,
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
    pub fn get_pi_value(&self, node: &Proposition, outcome: usize) -> Option<f64> {
        let key = (node.search_string(), outcome);
        self.pi_values.get(&key).cloned()
    }

    // Setter for pi values
    pub fn set_pi_value(&mut self, node: &Proposition, outcome: usize, value: f64) {
        let key = (node.search_string(), outcome);
        self.pi_values.insert(key, value);
    }

    // Getter for lambda values
    pub fn get_lambda_value(&self, node: &Proposition, outcome: usize) -> Option<f64> {
        let key = (node.search_string(), outcome);
        self.lambda_values.get(&key).cloned()
    }

    // Setter for lambda values
    pub fn set_lambda_value(&mut self, node: &Proposition, outcome: usize, value: f64) {
        let key = (node.search_string(), outcome);
        self.lambda_values.insert(key, value);
    }

    // Getter for pi messages
    pub fn get_pi_message(
        &self,
        from: &Proposition,
        to: &Proposition,
        outcome: usize,
    ) -> Option<f64> {
        let key = (from.search_string(), to.search_string(), outcome);
        self.pi_messages.get(&key).cloned()
    }

    // Setter for pi messages
    pub fn set_pi_message(
        &mut self,
        from: &Proposition,
        to: &Proposition,
        outcome: usize,
        value: f64,
    ) {
        let key = (from.search_string(), to.search_string(), outcome);
        self.pi_messages.insert(key, value);
    }

    // Getter for lambda messages
    pub fn get_lambda_message(
        &self,
        from: &Proposition,
        to: &Proposition,
        outcome: usize,
    ) -> Option<f64> {
        let key = (from.search_string(), to.search_string(), outcome);
        self.lambda_messages.get(&key).cloned()
    }

    // Setter for lambda messages
    pub fn set_lambda_message(
        &mut self,
        from: &Proposition,
        to: &Proposition,
        outcome: usize,
        value: f64,
    ) {
        let key = (from.search_string(), to.search_string(), outcome);
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