use std::error::Error;

use crate::model::objects::{PredicateConjunction, PredicateImplication, Predicate, Proposition};

use super::{graph::Graph, model::GraphicalModel, train::TrainingPlan, redis::ConnectionFactory};

pub struct TrainStatistics {
    pub loss: f64,
}
pub struct PredictStatistics {
    pub marginal: f64,
}

pub trait FactDB {
    fn get_proposition_probability(
        &self,
        proposition: &Proposition,
    ) -> Result<Option<f64>, Box<dyn Error>>;

    fn store_proposition_probability(
        &mut self,
        proposition: &Proposition,
        probability: f64,
    ) -> Result<(), Box<dyn Error>>;
}

pub trait ScenarioMaker {
    fn setup_scenario(
        &self,
        redis: &ConnectionFactory,
    ) -> Result<(), Box<dyn Error>>;
}
