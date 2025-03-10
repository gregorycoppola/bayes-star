use std::error::Error;

use redis::Connection;

use crate::model::objects::{PredicateGroup, ImplicationFactor, Predicate, Proposition};

use super::{graph::InferenceGraph, model::InferenceModel, train::TrainingPlan, redis::RedisManager, resources::ResourceContext};

pub struct TrainStatistics {
    pub loss: f64,
}

pub struct PredictStatistics {
    pub probability: f64,
}

pub trait BeliefTable {
    fn get_proposition_probability(
        &self,
        context: &mut Connection,
        proposition: &Proposition,
    ) -> Result<Option<f64>, Box<dyn Error>>;

    fn store_proposition_probability(
        &self,
        context: &mut Connection,
        proposition: &Proposition,
        probability: f64,
    ) -> Result<(), Box<dyn Error>>;

    fn store_proposition_boolean(
        &self,
        context: &mut Connection,
        proposition: &Proposition,
        observation: bool,
    ) -> Result<(), Box<dyn Error>> {
        if observation {
            self.store_proposition_probability(context, proposition, 1.0)?;
        } else {
            self.store_proposition_probability(context, proposition, 0.0)?;
        }
        Ok(())
    }
}

pub trait ScenarioMaker {
    fn setup_scenario(
        &self,
        redis: &ResourceContext,
    ) -> Result<(), Box<dyn Error>>;
}
