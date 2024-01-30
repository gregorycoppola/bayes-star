use std::error::Error;

use crate::model::objects::{PredicateGroup, PredicateFactor, Predicate, Proposition};

use super::{graph::InferenceGraph, model::InferenceModel, train::TrainingPlan, redis::RedisManager, resources::FactoryResources};

pub struct TrainStatistics {
    pub loss: f64,
}
pub struct PredictStatistics {
    pub probability: f64,
}

pub trait BeliefTable {
    fn get_proposition_probability(
        &self,
        proposition: &Proposition,
    ) -> Result<Option<f64>, Box<dyn Error>>;

    // Note: These methods use immutable self reference, but a `store`. Idea is it handles its own sync to write db.
    fn store_proposition_probability(
        &self,
        proposition: &Proposition,
        probability: f64,
    ) -> Result<(), Box<dyn Error>>;
    fn store_proposition_boolean(
        &self,
        proposition: &Proposition,
        observation: bool,
    ) -> Result<(), Box<dyn Error>> {
        if observation {
            self.store_proposition_probability(proposition, 1.0)?;
        } else {
            self.store_proposition_probability(proposition, 0.0)?;
        }
        Ok(())
    }
}

pub trait ScenarioMaker {
    fn setup_scenario(
        &self,
        redis: &FactoryResources,
    ) -> Result<(), Box<dyn Error>>;
}
