use crate::{
    common::interface::FactDB,
    model::{
        self,
        maxent::ExponentialModel,
        objects::{PredicateGroup, Domain, Entity, PredicateInferenceFactor, Predicate, Proposition},
    }, inference::graph::PropositionFactor,
};
use redis::{Commands, Connection};
use std::{cell::RefCell, error::Error, collections::HashMap};

use super::{
    interface::{PredictStatistics, TrainStatistics},
    redis::RedisManager, graph::InferenceGraph, fact_db::RedisFactDB, resources::FactoryResources,
};

pub struct GraphicalModel {
    pub graph: Box<InferenceGraph>,
    pub model: Box<dyn FactorModel>,
    pub fact_db: Box<dyn FactDB>,
}

impl GraphicalModel {
    pub fn new(resources: &FactoryResources) -> Result<Self, Box<dyn Error>> {
        let graph = InferenceGraph::new_mutable(resources)?;
        let model = ExponentialModel::new_mutable(&resources)?;
        let fact_db = RedisFactDB::new_mutable(&resources.redis)?;
        Ok(GraphicalModel {
            graph,
            model,
            fact_db,
        })
    }
}

#[derive(Debug)]
pub struct  FactorContext{
    pub factor: PropositionFactor,
    pub conjoined_probabilities: Vec<f64>,
}

pub trait FactorModel {
    fn initialize_connection(&mut self, implication: &PredicateInferenceFactor) -> Result<(), Box<dyn Error>>;
    fn train(
        &mut self,
        factor: &FactorContext,
        probability: f64,
    ) -> Result<TrainStatistics, Box<dyn Error>>;
    fn predict(&self, factor: &FactorContext) -> Result<PredictStatistics, Box<dyn Error>>;
}