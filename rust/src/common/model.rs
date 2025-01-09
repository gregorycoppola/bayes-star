use crate::{
    common::interface::BeliefTable,
    inference::graph::PropositionFactor,
    model::{
        self,
        exponential::ExponentialModel,
        objects::{
            Domain, Entity, Predicate, PredicateGroup, ImplicationFactor, Proposition,
        },
    },
};
use redis::{Commands, Connection};
use std::{cell::RefCell, collections::HashMap, error::Error, rc::Rc};

use super::{
    proposition_db::RedisBeliefTable,
    graph::InferenceGraph,
    interface::{PredictStatistics, TrainStatistics},
    redis::RedisManager,
    resources::NamespaceBundle,
};

pub struct InferenceModel {
    pub graph: Rc<InferenceGraph>,
    pub model: Rc<dyn FactorModel>,
}

impl InferenceModel {
    pub fn new_shared(resources: &NamespaceBundle) -> Result<Rc<Self>, Box<dyn Error>> {
        let graph = InferenceGraph::new_shared(resources.redis.get_arc_mutex_guarded_connection()?, resources.config.scenario_name.clone())?;
        let model = ExponentialModel::new_shared(&resources)?;
        Ok(Rc::new(InferenceModel {
            graph,
            model,
        }))
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
        implication: &ImplicationFactor,
    ) -> Result<(), Box<dyn Error>>;
    fn train(
        &mut self,
        factor: &FactorContext,
        probability: f64,
    ) -> Result<TrainStatistics, Box<dyn Error>>;
    fn predict(&self, factor: &FactorContext) -> Result<PredictStatistics, Box<dyn Error>>;
}
