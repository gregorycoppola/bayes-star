use std::error::Error;

use crate::model::objects::{Conjunction, Implication, Predicate};

use super::{graph::Graph, model::GraphicalModel, train::TrainingPlan, redis::RedisClient};

pub struct TrainStatistics {
    pub loss: f64,
}
pub struct PredictStatistics {
    pub marginal: f64,
}

pub trait FactDB {
    fn get_proposition_probability(
        &self,
        proposition: &Predicate,
    ) -> Result<Option<f64>, Box<dyn Error>>;

    fn store_proposition_probability(
        &mut self,
        proposition: &Predicate,
        probability: f64,
    ) -> Result<(), Box<dyn Error>>;
}

pub trait ScenarioMaker {
    fn setup_scenario(
        &self,
        redis: &RedisClient,
    ) -> Result<(), Box<dyn Error>>;
}
