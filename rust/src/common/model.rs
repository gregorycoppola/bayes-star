use crate::{
    common::interface::FactDB,
    model::{
        self,
        maxent::ExponentialModel,
        objects::{Conjunct, Domain, Entity, ImplicationLink, Proposition, ConjunctLink},
    },
};
use redis::{Commands, Connection};
use std::{cell::RefCell, error::Error};

use super::{
    interface::{PredictStatistics, TrainStatistics},
    redis::RedisClient, graph::Graph, fact_db::RedisFactDB,
};

pub struct GraphicalModel {
    pub graph: Graph,
    pub model: Box<dyn FactorModel>,
    pub fact_db: Box<dyn FactDB>,
}

impl GraphicalModel {
    pub fn new(_model_spec: &String, redis_client: &RedisClient) -> Result<Self, Box<dyn Error>> {
        let graph_connection = redis_client.get_connection()?;
        let model_connection = redis_client.get_connection()?;
        let graph = Graph::new(graph_connection)?;
        let model = ExponentialModel::new(model_connection)?;
        let fact_db = RedisFactDB::new(redis_client)?;
        Ok(GraphicalModel {
            graph,
            model,
            fact_db,
        })
    }
}

#[derive(Debug)]
pub struct  Factor {
    pub conjuncts: Vec<ConjunctLink>,
    pub conclusion: Proposition,
}

#[derive(Debug)]
pub struct  FactorContext{
    pub factor: Factor,
    pub conjunct_probabilities: Vec<f64>,
}

pub trait FactorModel {
    fn initialize_connection(&mut self, link: &ImplicationLink) -> Result<(), Box<dyn Error>>;
    fn train(
        &mut self,
        factor: &FactorContext,
        probability: f64,
    ) -> Result<TrainStatistics, Box<dyn Error>>;
    fn predict(&self, factor: &FactorContext) -> Result<PredictStatistics, Box<dyn Error>>;
}