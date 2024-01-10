use std::error::Error;

use crate::model::objects::{Implication, Proposition, Conjunction};

use super::model::GraphicalModel;

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
    fn get_conjunction_probability(
        &self,
        conjunction: &Conjunction,
    ) -> Result<Option<f64>, Box<dyn Error>>;
}

pub trait ScenarioMaker {
    fn setup_scenario(&self, storage: &mut GraphicalModel) -> Result<(), Box<dyn Error>>;
}
