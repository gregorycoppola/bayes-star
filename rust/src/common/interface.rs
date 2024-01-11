use std::error::Error;

use crate::model::objects::{Conjunct, ImplicationLink, Proposition};

use super::{graph::Graph, model::GraphicalModel, train::TrainingPlan};

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
        graph: &mut Graph,
        fact_db: &mut dyn FactDB,
        training_plan: &mut TrainingPlan,
    ) -> Result<(), Box<dyn Error>>;
}
