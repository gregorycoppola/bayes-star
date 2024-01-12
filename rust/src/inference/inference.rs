use super::table::{HashMapBeliefTable, InferenceNode, InferenceResult};
use crate::{
    common::{interface::FactDB, model::GraphicalModel},
    model::{
        objects::{Conjunction, Proposition},
        weights::CLASS_LABELS,
    },
};
use redis::Connection;
use std::{borrow::Borrow, collections::HashMap, error::Error};

struct Inferencer {
    model: Box<GraphicalModel>,
    evidence: Box<dyn FactDB>,
    data: HashMapBeliefTable,
}

fn inference_proposition_probability(
    fact_db: &dyn FactDB,
    proposition: &Proposition,
) -> Result<f64, Box<dyn Error>> {
    todo!()
}

fn inference_conjunct_probability(
    fact_db: &dyn FactDB,
    conjunct: &Conjunction,
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
            data: HashMapBeliefTable::new(),
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
            self.initialize_pi_proposition(root, true)?;
        }
        Ok(())
    }

    pub fn initialize_pi_proposition(
        &mut self,
        node: &Proposition,
        is_root: bool,
    ) -> Result<(), Box<dyn Error>> {
        let children = self.model.graph.children_of_proposition(node)?;
        for child in &children {
            self.initialize_pi_conjunct(child, false)?;
        }
        if is_root {
            let prior_prob = inference_proposition_probability(self.model.fact_db.borrow(), node)?;
            self.data
                .set_pi_value(&InferenceNode::from_proposition(node), 1, prior_prob);
            self.data.set_pi_value(
                &InferenceNode::from_proposition(node),
                0,
                1f64 - prior_prob,
            );
        }
        for outcome in CLASS_LABELS {
            let children = self
                .model
                .graph
                .children_of_proposition(node)
                .expect("Error finding children");
            for child in &children {
                self.data.set_lambda_message(
                    &InferenceNode::from_proposition(node),
                    &InferenceNode::from_conjunct(child),
                    outcome,
                    1f64,
                );
            }
        }
        Ok(())
    }

    pub fn initialize_pi_conjunct(
        &mut self,
        conjunct: &Conjunction,
        is_root: bool,
    ) -> Result<(), Box<dyn Error>> {
        let children = self.model.graph.children_of_conjunct(conjunct)?;
        for child in &children {
            self.initialize_pi_proposition(child, false)?;
        }
        if is_root {
            let prior_prob = inference_conjunct_probability(self.model.fact_db.borrow(), conjunct)?;
            self.data
                .set_pi_value(&InferenceNode::from_conjunct(conjunct), 1, prior_prob);
            self.data.set_pi_value(
                &InferenceNode::from_conjunct(conjunct),
                0,
                1f64 - prior_prob,
            );
        }
        for outcome in CLASS_LABELS {
            let children = self
                .model
                .graph
                .children_of_conjunct(conjunct)
                .expect("Error finding children");
            for child in &children {
                self.data.set_lambda_message(
                    &InferenceNode::from_conjunct(conjunct),
                    &InferenceNode::from_proposition(child),
                    outcome,
                    1f64,
                );
            }
        }
        Ok(())
    }

    pub fn initialize_lambda(&mut self) -> Result<(), Box<dyn Error>> {
        let roots = self.model.graph.find_roots()?;
        for root in &roots {
            self.initialize_lambda_proposition(root)?;
        }
        Ok(())
    }

    pub fn initialize_lambda_proposition(
        &mut self,
        node: &Proposition,
    ) -> Result<(), Box<dyn Error>> {
        for outcome in CLASS_LABELS {
            self.data
                .set_lambda_value(&InferenceNode::from_proposition(node), outcome, 1f64);
            let parents = self.model.graph.parents_of_proposition(node)?;
            for parent in &parents {
                self.data.set_lambda_message(
                    &InferenceNode::from_proposition(node),
                    &InferenceNode::from_conjunct(parent),
                    outcome,
                    1f64,
                );
            }
        }
        let children = self.model.graph.children_of_proposition(node)?;
        for child in &children {
            self.initialize_lambda_conjunct(child)?;
        }
        Ok(())
    }

    pub fn initialize_lambda_conjunct(
        &mut self,
        conjunct: &Conjunction,
    ) -> Result<(), Box<dyn Error>> {
        for outcome in CLASS_LABELS {
            self.data
                .set_lambda_value(&InferenceNode::from_conjunct(conjunct), outcome, 1f64);
            let parents = self.model.graph.parents_of_conjunct(conjunct)?;
            for parent in &parents {
                self.data.set_lambda_message(
                    &InferenceNode::from_conjunct(conjunct),
                    &InferenceNode::from_proposition(parent),
                    outcome,
                    1f64,
                );
            }
        }
        let children = self.model.graph.children_of_conjunct(conjunct)?;
        for child in &children {
            self.initialize_lambda_proposition(child)?;
        }
        Ok(())
    }
}

pub fn compute_marginals(
    model: Box<GraphicalModel>,
    evidence: Box<dyn FactDB>,
) -> Result<Box<dyn InferenceResult>, Box<dyn Error>> {
    todo!()
}
