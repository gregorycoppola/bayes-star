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
    redis::RedisClient, graph::Graph,
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

pub struct RedisFactDB {
    redis_connection: RefCell<redis::Connection>,
}

impl RedisFactDB {
    pub fn new(client: &RedisClient) -> Result<Box<dyn FactDB>, Box<dyn Error>> {
        let redis_connection = client.get_connection()?;
        Ok(Box::new(RedisFactDB { redis_connection }))
    }
}

impl FactDB for RedisFactDB {
    // Return Some if the probability exists in the table, or else None.
    fn get_proposition_probability(
        &self,
        proposition: &Proposition,
    ) -> Result<Option<f64>, Box<dyn Error>> {
        let search_string = proposition.search_string();

        // Use a match statement to handle the different outcomes
        match self
            .redis_connection
            .borrow_mut()
            .hget::<_, _, String>("probs", &search_string)
        {
            Ok(probability_str) => {
                // Found the entry, parse it
                let probability = probability_str
                    .parse::<f64>()
                    .map_err(|e| Box::new(e) as Box<dyn Error>)?;
                Ok(Some(probability))
            }
            Err(e) => {
                // Handle specific "not found" error
                if e.kind() == redis::ErrorKind::TypeError {
                    // Entry not found in Redis
                    Ok(None)
                } else {
                    // Other Redis errors
                    Err(Box::new(e) as Box<dyn Error>)
                }
            }
        }
    }
}
