use crate::{
    common::{interface::FactDB, redis::seq_get_all},
    model::{
        self,
        maxent::ExponentialModel,
        objects::{
            Domain, Entity, PredicateInferenceFactor, Predicate, PredicateGroup,
            Proposition,
        },
    },
};
use redis::{Commands, Connection};
use serde::Deserialize;
use std::{cell::RefCell, error::Error};

use super::{
    interface::{PredictStatistics, TrainStatistics},
    redis::RedisManager,
};

pub struct TrainingPlan {
    redis_connection: RefCell<redis::Connection>,
}

impl TrainingPlan {
    pub fn new(redis: &RedisManager) -> Result<Self, Box<dyn Error>> {
        let redis_connection = redis.get_connection()?;
        Ok(TrainingPlan { redis_connection })
    }

    pub fn add_proposition_to_queue(
        &mut self,
        queue_name: &String,
        proposition: &Proposition,
    ) -> Result<(), Box<dyn Error>> {
        info!(
            "GraphicalModel::add_to_training_queue - Start. Input proposition: {:?}",
            proposition
        );
        let serialized_proposition = match serde_json::to_string(proposition) {
            Ok(record) => record,
            Err(e) => {
                info!(
                    "GraphicalModel::add_to_training_queue - Error serializing proposition: {}",
                    e
                );
                return Err(Box::new(e));
            }
        };
        info!(
            "GraphicalModel::add_to_training_queue - Serialized proposition: {}",
            &serialized_proposition
        );
        if let Err(e) = self
            .redis_connection
            .borrow_mut()
            .rpush::<_, _, bool>(queue_name, &serialized_proposition)
        {
            info!("GraphicalModel::add_to_training_queue - Error adding proposition to training queue in Redis: {}", e);
            return Err(Box::new(e));
        }
        info!("GraphicalModel::add_to_training_queue - Proposition added to training queue successfully");
        Ok(())
    }

    pub fn maybe_add_to_training(
        &mut self,
        is_training: bool,
        proposition: &Proposition,
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
        proposition: &Proposition,
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
        seq_name: &String,
    ) -> Result<Vec<Proposition>, Box<dyn Error>> {
        info!(
            "GraphicalModel::get_propositions_from_queue - Start. Queue name: {}",
            seq_name
        );
        let records = seq_get_all(&mut self.redis_connection.borrow_mut(), &seq_name)?;
        let mut result = vec![];
        for record in &records {
            let proposition = deserialize_record(record)?;
            result.push(proposition);
        }
        info!("GraphicalModel::get_propositions_from_queue - Retrieved and deserialized propositions successfully");
        Ok(result)
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

fn deserialize_record<'a, T>(record: &'a str) -> Result<T, Box<dyn Error>>
where
    T: Deserialize<'a>,
{
    serde_json::from_str(record).map_err(|e| Box::new(e) as Box<dyn Error>)
}
