use crate::{
    common::interface::FactDB,
    model::{
        self,
        maxent::ExponentialModel,
        objects::{PredicateGroup, Domain, Entity, PredicateInferenceFactor, Predicate, Proposition},
    },
};
use redis::{Commands, Connection};
use std::{cell::RefCell, error::Error};

use super::{
    interface::{PredictStatistics, TrainStatistics},
    redis::RedisManager, graph::InferenceGraph,
};

pub struct RedisFactDB {
    redis_connection: RefCell<redis::Connection>,
}

impl RedisFactDB {
    pub fn new_mutable(client: &RedisManager) -> Result<Box<dyn FactDB>, Box<dyn Error>> {
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
        let hash_string = proposition.predicate.hash_string();

        // Use a match statement to handle the different outcomes
        match self
            .redis_connection
            .borrow_mut()
            .hget::<_, _, String>("probs", &hash_string)
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

    fn store_proposition_probability(
        &mut self,
        proposition: &Proposition,
        probability: f64,
    ) -> Result<(), Box<dyn Error>> {
        trace!("GraphicalModel::store_proposition_probability - Start. Input proposition: {:?}, probability: {}", proposition, probability);

        let hash_string = proposition.predicate.hash_string();
        trace!(
            "GraphicalModel::store_proposition_probability - Computed hash_string: {}",
            hash_string
        );

        if let Err(e) = self
            .redis_connection
            .borrow_mut()
            .hset::<&str, &str, String, bool>("probs", &hash_string, probability.to_string())
        {
            trace!(
                "GraphicalModel::store_proposition_probability - Error storing probability in Redis: {}",
                e
            );
            return Err(Box::new(e));
        }

        trace!("GraphicalModel::store_proposition_probability - Completed successfully");
        Ok(())
    }

}
