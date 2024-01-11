
use std::{collections::HashMap, error::Error};
use redis::Connection;
use crate::{model::{objects::Proposition, weights::CLASS_LABELS}, common::{interface::FactDB, model::{GraphicalModel, Graph}}};
use super::table::BeliefPropagationData;

struct Inferencer {
    model: Box<GraphicalModel>,
    evidence: Box<dyn FactDB>,
    data: BeliefPropagationData,
}

fn get_proposition_probability_combined(model_db: &dyn FactDB, evidence_db:&dyn FactDB) -> Result<f64, Box<dyn Error>> {
    todo!()
}

impl Inferencer {
    // Initialize new Storage with a Redis connection
    pub fn new(model:Box<GraphicalModel>, evidence: Box<dyn FactDB>) -> Result<Self, redis::RedisError> {
        Ok(Inferencer {
            model,
            evidence,
            data: BeliefPropagationData::new(),
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

pub fn compute_marginals(model:Box<GraphicalModel>, evidence:Box<dyn FactDB>) -> Result<Box<dyn FactDB>, Box<dyn Error>> {
    todo!()
}