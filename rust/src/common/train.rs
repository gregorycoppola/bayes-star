use crate::{
    common::interface::FactDB,
    model::{
        self,
        maxent::ExponentialModel,
        objects::{
            Domain, Entity, Implication, ImplicationInstance, Predicate, PredicateConjunction,
            Proposition,
        },
    },
};
use redis::{Commands, Connection};
use std::{cell::RefCell, error::Error};

use super::{
    interface::{PredictStatistics, TrainStatistics},
    redis::RedisClient,
};

pub struct TrainingPlan {
    redis_connection: RefCell<redis::Connection>,
}

impl TrainingPlan {
    pub fn new(redis: &RedisClient) -> Result<Self, Box<dyn Error>> {
        let redis_connection = redis.get_connection()?;
        Ok(TrainingPlan { redis_connection })
    }

    pub fn add_proposition_to_queue(
        &mut self,
        queue_name: &String,
        proposition: &Predicate,
    ) -> Result<(), Box<dyn Error>> {
        trace!(
            "GraphicalModel::add_to_training_queue - Start. Input proposition: {:?}",
            proposition
        );

        let serialized_proposition = match serde_json::to_string(proposition) {
            Ok(record) => record,
            Err(e) => {
                trace!(
                    "GraphicalModel::add_to_training_queue - Error serializing proposition: {}",
                    e
                );
                return Err(Box::new(e));
            }
        };
        trace!(
            "GraphicalModel::add_to_training_queue - Serialized proposition: {}",
            &serialized_proposition
        );

        if let Err(e) = self
            .redis_connection
            .borrow_mut()
            .rpush::<_, _, bool>(queue_name, &serialized_proposition)
        {
            trace!("GraphicalModel::add_to_training_queue - Error adding proposition to training queue in Redis: {}", e);
            return Err(Box::new(e));
        }

        trace!("GraphicalModel::add_to_training_queue - Proposition added to training queue successfully");

        Ok(())
    }

    pub fn maybe_add_to_training(
        &mut self,
        is_training: bool,
        proposition: &Predicate,
    ) -> Result<(), Box<dyn Error>> {
        if is_training {
            self.add_proposition_to_queue(&"training_queue".to_string(), &proposition)
        } else {
            Ok(())
        }
    }

    pub fn maybe_add_to_test(
        &mut self,
        is_test: bool,
        proposition: &Predicate,
    ) -> Result<(), Box<dyn Error>> {
        if is_test {
            self.add_proposition_to_queue(&"test_queue".to_string(), &proposition)
        } else {
            Ok(())
        }
    }

    // TODO: Right now this is consuming the queue.. should just be a vector, or an interator.
    fn get_propositions_from_queue(
        &self,
        queue_name: &String,
    ) -> Result<Vec<Proposition>, Box<dyn Error>> {
        trace!(
            "GraphicalModel::get_propositions_from_queue - Start. Queue name: {}",
            queue_name
        );

        let mut propositions = Vec::new();

        // Attempt to pop one element at a time from the Redis queue
        while let Some(serialized_proposition) = self
            .redis_connection
            .borrow_mut()
            .lpop::<_, Option<String>>(queue_name, None)?
        {
            match serde_json::from_str(&serialized_proposition)
                .map_err(|e| Box::new(e) as Box<dyn Error>)
            {
                Ok(predicate) => {
                    let proposition = Proposition { predicate };
                    propositions.push(proposition)
                }
                Err(e) => {
                    trace!("GraphicalModel::get_propositions_from_queue - Error deserializing proposition: {}", e);
                    return Err(e);
                }
            }
        }

        trace!("GraphicalModel::get_propositions_from_queue - Retrieved and deserialized propositions successfully");

        Ok(propositions)
    }

    pub fn get_training_questions(&self) -> Result<Vec<Proposition>, Box<dyn Error>> {
        let training_queue_name = String::from("training_queue");
        self.get_propositions_from_queue(&training_queue_name)
    }

    pub fn get_test_questions(&self) -> Result<Vec<Proposition>, Box<dyn Error>> {
        let test_queue_name = String::from("test_queue");
        self.get_propositions_from_queue(&test_queue_name)
    }
}
