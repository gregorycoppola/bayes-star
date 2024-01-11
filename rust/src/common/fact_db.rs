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
