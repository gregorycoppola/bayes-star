use std::error::Error;

use crate::model::{
    objects::{Implication, Proposition},
    storage::GraphicalModel,
};

pub struct TrainStatistics {
    pub loss: f64,
}
pub struct PredictStatistics {
    pub marginal: f64,
}

pub trait PropositionProbability {
    fn get_proposition_probability(
        &self,
        proposition: &Proposition,
    ) -> Result<Option<f64>, Box<dyn Error>>;
}

pub trait FactorModel {
    fn initialize_connection(&mut self, implication: &Implication) -> Result<(), Box<dyn Error>>;
    fn train(
        &mut self,
        storage: &GraphicalModel,
        proposition: &Proposition,
    ) -> Result<TrainStatistics, Box<dyn Error>>;
    fn predict(
        &self,
        storage: &GraphicalModel,
        evidence: &dyn PropositionProbability,
        proposition: &Proposition,
    ) -> Result<PredictStatistics, Box<dyn Error>>;
}

pub trait ScenarioMaker {
    fn setup_scenario(&self, storage: &mut GraphicalModel) -> Result<(), Box<dyn Error>>;
}
