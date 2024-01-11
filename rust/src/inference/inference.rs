use super::table::BeliefPropagationData;
use crate::{
    common::{
        interface::FactDB,
        model::{Graph, GraphicalModel},
    },
    model::{objects::Proposition, weights::CLASS_LABELS},
};
use redis::Connection;
use std::{borrow::Borrow, collections::HashMap, error::Error};

struct Inferencer {
    model: Box<GraphicalModel>,
    evidence: Box<dyn FactDB>,
    data: BeliefPropagationData,
}

fn get_proposition_probability_combined(
    model_db: &dyn FactDB,
    evidence_db: &dyn FactDB,
    proposition: &Proposition,
) -> Result<f64, Box<dyn Error>> {
    todo!()
}

impl Inferencer {
    // Initialize new Storage with a Redis connection
    pub fn new(
        model: Box<GraphicalModel>,
        evidence: Box<dyn FactDB>,
    ) -> Result<Self, redis::RedisError> {
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
        let roots = self.model.graph.find_roots()?;
        for root in &roots {
            self.initialize_pi_node(root, true)?;
        }
        Ok(())
    }

    pub fn initialize_pi_node(
        &mut self,
        node: &Proposition,
        is_root: bool,
    ) -> Result<(), Box<dyn Error>> {
        let children = self.model.graph.find_children(node)?;
        for child in &children {
            self.initialize_pi_node(child, false)?;
        }

        if is_root {
            let prior_prob = get_proposition_probability_combined(
                self.model.fact_db.borrow(),
                self.evidence.borrow(),
                node,
            )?;
            self.data.set_pi_value(node, 1, prior_prob);
            self.data.set_pi_value(node, 0, 1f64 - prior_prob);
        }
        for outcome in CLASS_LABELS {
            let children = self.model.graph.find_children(node).expect("Error finding children");
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

    pub fn initialize_lambda_node(&mut self, node: &Proposition) -> Result<(), Box<dyn Error>> {
        for outcome in CLASS_LABELS {
            self.data.set_lambda_value(node, outcome, 1f64);
            let parents = self.model.graph
                .find_parents(node)?;
            for parent in &parents {
                self.data.set_lambda_message(node, parent, outcome, 1f64);
            }
        }
        let children = self.model.graph.find_children(node)?;
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
}

pub fn compute_marginals(
    model: Box<GraphicalModel>,
    evidence: Box<dyn FactDB>,
) -> Result<Box<dyn FactDB>, Box<dyn Error>> {
    todo!()
}
