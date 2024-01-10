use std::error::Error;

use crate::model::{storage::Storage, objects::Proposition};

pub struct TrainStatistics {
    loss: f64,
}
pub struct PredictStatistics {
    marginal:f64,
}

pub trait PropositionProbability {
    fn get_proposition_probability(
        &mut self,
        proposition: &Proposition,
    ) -> Result<Option<f64>, Box<dyn Error>>;
}

pub trait LogicalModel {

    fn train(storage:&mut Storage, proposition:&Proposition) -> Result<TrainStatistics, Box<dyn Error>>;
    fn predict(storage:&mut Storage, evidence:&PropositionProbability, proposition:&Proposition) -> Result<PredictStatistics, Box<dyn Error>>;

}