use crate::{
    common::interface::BeliefTable,
    inference::graph::PropositionFactor,
    model::{
        self,
        exponential::ExponentialModel,
        objects::{Domain, Entity, ImplicationFactor, Predicate, PredicateGroup, Proposition},
    },
};
use redis::{Commands, Connection};
use std::{cell::RefCell, collections::HashMap, error::Error, rc::Rc, sync::Arc};

use super::{
    graph::InferenceGraph,
    interface::{PredictStatistics, TrainStatistics},
    proposition_db::RedisBeliefTable,
    redis::RedisManager,
    resources::ResourceContext,
};

pub struct InferenceModel {
    pub graph: Arc<InferenceGraph>,
    pub model: Arc<dyn FactorModel>,
}

impl InferenceModel {
    pub fn new_shared(namespace: String) -> Result<Arc<Self>, Box<dyn Error>> {
        let graph = InferenceGraph::new_shared(namespace.clone())?;
        let model = ExponentialModel::new_shared(namespace.clone())?;
        Ok(Arc::new(InferenceModel { graph, model }))
    }
}

#[derive(Debug)]
pub struct FactorContext {
    pub factor: Vec<PropositionFactor>,
    pub probabilities: Vec<f64>,
}

pub trait FactorModel {
    fn initialize_connection(
        &mut self,
        connection: &mut Connection,
        implication: &ImplicationFactor,
    ) -> Result<(), Box<dyn Error>>;

    fn train(
        &mut self,
        connection: &mut Connection,
        factor: &FactorContext,
        probability: f64,
    ) -> Result<TrainStatistics, Box<dyn Error>>;

    fn predict(
        &self,
        connection: &mut Connection,
        factor: &FactorContext,
    ) -> Result<PredictStatistics, Box<dyn Error>>;
}
