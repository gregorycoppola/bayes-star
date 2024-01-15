use crate::{
    common::interface::PropositionDB,
    inference::graph::PropositionFactor,
    model::{
        self,
        maxent::ExponentialModel,
        objects::{
            Domain, Entity, Predicate, PredicateGroup, PredicateInferenceFactor, Proposition,
        },
    },
};
use redis::{Commands, Connection};
use std::{cell::RefCell, collections::HashMap, error::Error, rc::Rc};

use super::{
    proposition_db::RedisFactDB,
    graph::InferenceGraph,
    interface::{PredictStatistics, TrainStatistics},
    redis::RedisManager,
    resources::FactoryResources,
};

pub struct GraphicalModel {
    pub graph: Rc<InferenceGraph>,
    pub model: Rc<dyn FactorModel>,
    pub proposition_db: Rc<dyn PropositionDB>,
}

impl GraphicalModel {
    pub fn new_shared(resources: &FactoryResources) -> Result<Rc<Self>, Box<dyn Error>> {
        let graph = InferenceGraph::new_shared(resources)?;
        let model = ExponentialModel::new_shared(&resources)?;
        let proposition_db = RedisFactDB::new_shared(&resources.redis)?;
        Ok(Rc::new(GraphicalModel {
            graph,
            model,
            proposition_db,
        }))
    }
}

#[derive(Debug)]
pub struct FactorContext {
    pub factor: PropositionFactor,
    pub probabilities: Vec<f64>,
}

pub trait FactorModel {
    fn initialize_connection(
        &mut self,
        implication: &PredicateInferenceFactor,
    ) -> Result<(), Box<dyn Error>>;
    fn train(
        &mut self,
        factor: &FactorContext,
        probability: f64,
    ) -> Result<TrainStatistics, Box<dyn Error>>;
    fn predict(&self, factor: &FactorContext) -> Result<PredictStatistics, Box<dyn Error>>;
}
