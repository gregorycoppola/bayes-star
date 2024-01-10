use std::error::Error;

use crate::model::{objects::Proposition, storage::Storage};

pub struct TrainStatistics {
    pub loss: f64,
}
pub struct PredictStatistics {
    pub marginal: f64,
}

pub trait PropositionProbability {
    fn get_proposition_probability(
        &mut self,
        proposition: &Proposition,
    ) -> Result<Option<f64>, Box<dyn Error>>;
}

pub trait LogicalModel {
    fn train(
        storage: &mut Storage,
        proposition: &Proposition,
    ) -> Result<TrainStatistics, Box<dyn Error>>;
    fn predict(
        storage: &mut Storage,
        evidence: &dyn PropositionProbability,
        proposition: &Proposition,
    ) -> Result<PredictStatistics, Box<dyn Error>>;
}


pub trait ScenarioMaker {
    fn setup_scenario(&self, storage:&mut Storage) -> Result<(), Box<dyn Error>>;
}