use std::{collections::HashMap, error::Error};

use redis::Connection;

use crate::model::{objects::Proposition, weights::CLASS_LABELS, storage::PropositionProbability};

pub struct BeliefPropagationData {
    pi_values: HashMap<(String, usize), f64>,
    lambda_values: HashMap<(String, usize), f64>,
    pi_messages: HashMap<(String, String, usize), f64>,
    lambda_messages: HashMap<(String, String, usize), f64>,
}

impl BeliefPropagationData {
    // Constructor to create a new instance
    pub fn new() -> Self {
        BeliefPropagationData {
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

pub struct BeliefPropagator {
    data: BeliefPropagationData,
    evidence: Box<dyn PropositionProbability>,
}

impl BeliefPropagator {
    // Initialize new Storage with a Redis connection
    pub fn new(evidence: Box<dyn PropositionProbability>) -> Result<Self, redis::RedisError> {
        Ok(BeliefPropagator {
            data: BeliefPropagationData::new(), evidence,
        })
    }

    pub fn initialize(&mut self) -> Result<(), Box<dyn Error>> {
        self.initialize_pi()?;
        self.initialize_lambda()?;
        Ok(())
    }

    pub fn initialize_pi(&mut self) -> Result<(), Box<dyn Error>> {
        let root = self.find_root()?;
        self.initialize_pi_node(&root, true)?;
        Ok(())
    }

    pub fn initialize_pi_node(&mut self, node:&Proposition, is_root:bool) -> Result<(), Box<dyn Error>> {
        let children = self.find_children(node)?;
        for child in &children {
            self.initialize_pi_node(child, false)?;
        }

        if is_root {
            let prior_prob = self.get_proposition_probability(node)?;
            self.data.set_pi_value(node, 1, prior_prob);
            self.data.set_pi_value(node, 0, 1f64 - prior_prob);
        }
        for outcome in CLASS_LABELS {
            let children =  self.find_children(node).expect("Error finding children");
            for child in &children {
                self.data.set_lambda_message(node, child, outcome, 1f64);
            }
        }
    
        Ok(())
    }


    pub fn initialize_lambda(&mut self) -> Result<(), Box<dyn Error>> {
        let root = self.find_root()?;
        self.initialize_lambda_node(&root)?;
        Ok(())
    }

    pub fn initialize_lambda_node(&mut self, node:&Proposition) -> Result<(), Box<dyn Error>> {
        for outcome in CLASS_LABELS {
            self.data.set_lambda_value(node, outcome, 1f64);
            let parents =  &self.find_parent(node).expect("Error finding paerents").expect("No parents");
            for parent in &[parents] {
                self.data.set_lambda_message(node, parent, outcome, 1f64);
            }
        }
        let children = self.find_children(node)?;
        for child in &children {
            self.initialize_lambda_node(child)?;
        }
        Ok(())
    }

    pub fn get_all_propositions(&self) -> Result<Vec<Proposition>, Box<dyn Error>> {
        // Your implementation here
        todo!()
    }

    pub fn get_proposition_probability(
        &self,
        proposition: &Proposition,
    ) -> Result<f64, Box<dyn Error>> {
        // Your implementation here
        todo!()
    }

    pub fn get_conditional_probability(
        &self,
        conclusion: &Proposition,
        premise: &Proposition,
    ) -> Result<f64, Box<dyn Error>> {
        // Your implementation here
        todo!()
    }

    fn find_parent(&self, x: &Proposition) -> Result<Option<Proposition>, Box<dyn Error>> {
        // Your implementation here
        Ok(None) // Placeholder
    }

    fn find_root(&self) -> Result<Proposition, Box<dyn Error>> {
        // Your implementation here
        todo!()
    }

    fn find_children(&self, root: &Proposition) -> Result<Vec<Proposition>, Box<dyn Error>> {
        // Your implementation here
        Ok(Vec::new()) // Placeholder
    }
}
