use crate::{model::objects::{Domain, Entity, Implication, Proposition}, common::interface::PropositionProbability};
use redis::{Commands, Connection};
use std::{error::Error, cell::RefCell};

pub struct Storage {
    redis_connection: RefCell<redis::Connection>,
}

impl Drop for Storage {
    fn drop(&mut self) {
        // The Drop trait for Arc<Client> will automatically be called here,
        // reducing the reference count. If this Storage instance holds the last
        // reference to the client, the client will be dropped and its resources
        // (like network connections) will be cleaned up.
    }
}

impl Storage {
    // Initialize new Storage with a Redis connection
    pub fn new(connection: Connection) -> Result<Self, redis::RedisError> {
        Ok(Storage {
            redis_connection: RefCell::new(connection),
        })
    }
    pub fn drop_all_dbs(&mut self) -> Result<(), Box<dyn Error>> {
        redis::cmd("FLUSHDB").query(&mut self.redis_connection.borrow_mut())?;
        trace!("Database flushed successfully");

        Ok(())
    }

    // Store an entity
    pub fn store_entity(&mut self, entity: &Entity) -> Result<(), Box<dyn Error>> {
        trace!(
            "Storing entity in domain '{}': {}",
            entity.domain,
            entity.name
        ); // Logging
        self.redis_connection.borrow_mut()
            .sadd(&entity.domain.to_string(), &entity.name)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(())
    }

    pub fn get_entities_in_domain(&self, domain: &str) -> Result<Vec<Entity>, Box<dyn Error>> {
        trace!("Getting entities in domain '{}'", domain.clone()); // Logging

        let names: Vec<String> = self
            .redis_connection.borrow_mut()
            .smembers(domain)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(names
            .into_iter()
            .map(|name| Entity {
                domain: Domain::from_str(domain).expect("Domain not recognized."), // Use the provided domain
                name,
            })
            .collect())
    }

    pub fn store_proposition(
        &mut self,
        proposition: &Proposition,
        probability: f64,
    ) -> Result<(), Box<dyn Error>> {
        trace!(
            "Storage::store_proposition - Start. Input proposition: {:?}, probability: {}",
            proposition,
            probability
        );

        let search_string = proposition.search_string();
        trace!(
            "Storage::store_proposition - Computed search_string: {}",
            search_string
        );

        let record = match serde_json::to_string(proposition) {
            Ok(record) => record,
            Err(e) => {
                trace!(
                    "Storage::store_proposition - Error serializing proposition: {}",
                    e
                );
                return Err(Box::new(e));
            }
        };
        trace!(
            "Storage::store_proposition - Serialized proposition record: {} {}",
            &search_string,
            &record
        );

        if let Err(e) =
            self.redis_connection.borrow_mut()
                .hset::<_, _, _, bool>("propositions", &search_string, &record)
        {
            trace!(
                "Storage::store_proposition - Error storing proposition in Redis: {}",
                e
            );
            return Err(Box::new(e));
        }

        match self.store_proposition_probability(proposition, probability) {
            Ok(_) => trace!("Storage::store_proposition - Completed successfully"),
            Err(e) => trace!(
                "Storage::store_proposition - Error in store_proposition_probability: {}",
                e
            ),
        }

        Ok(())
    }

    pub fn store_proposition_probability(
        &mut self,
        proposition: &Proposition,
        probability: f64,
    ) -> Result<(), Box<dyn Error>> {
        trace!("Storage::store_proposition_probability - Start. Input proposition: {:?}, probability: {}", proposition, probability);

        let search_string = proposition.search_string();
        trace!(
            "Storage::store_proposition_probability - Computed search_string: {}",
            search_string
        );

        if let Err(e) = self.redis_connection.borrow_mut().hset::<&str, &str, String, bool>(
            "probs",
            &search_string,
            probability.to_string(),
        ) {
            trace!(
                "Storage::store_proposition_probability - Error storing probability in Redis: {}",
                e
            );
            return Err(Box::new(e));
        }

        trace!("Storage::store_proposition_probability - Completed successfully");
        Ok(())
    }

    pub fn store_implication(&mut self, implication: &Implication) -> Result<(), Box<dyn Error>> {
        let record =
            serde_json::to_string(implication).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        self.redis_connection.borrow_mut()
            .sadd("implications", &record)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        self.store_links(implication)
    }

    pub fn store_links(&mut self, implication: &Implication) -> Result<(), Box<dyn Error>> {
        let search_string = implication.search_string();
        let record =
            serde_json::to_string(implication).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        self.redis_connection.borrow_mut()
            .sadd(&search_string, &record)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(())
    }

    // Get all Implications
    pub fn get_all_implications(&self) -> Result<Vec<Implication>, Box<dyn Error>> {
        let all_values: Vec<String> = self
            .redis_connection.borrow_mut()
            .smembers("implications")
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        all_values
            .into_iter()
            .map(|record| serde_json::from_str(&record).map_err(|e| Box::new(e) as Box<dyn Error>))
            .collect()
    }

    pub fn find_premises(
        &self,
        search_string: &str,
    ) -> Result<Vec<Implication>, Box<dyn Error>> {
        trace!("find_premises: {:?}", &search_string);
        let set_members: Vec<String> = self
            .redis_connection.borrow_mut()
            .smembers(search_string)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        set_members
            .into_iter()
            .map(|record| serde_json::from_str(&record).map_err(|e| Box::new(e) as Box<dyn Error>))
            .collect()
    }

    pub fn add_proposition_to_queue(
        &mut self,
        queue_name: &String,
        proposition: &Proposition,
    ) -> Result<(), Box<dyn Error>> {
        trace!(
            "Storage::add_to_training_queue - Start. Input proposition: {:?}",
            proposition
        );

        let serialized_proposition = match serde_json::to_string(proposition) {
            Ok(record) => record,
            Err(e) => {
                trace!(
                    "Storage::add_to_training_queue - Error serializing proposition: {}",
                    e
                );
                return Err(Box::new(e));
            }
        };
        trace!(
            "Storage::add_to_training_queue - Serialized proposition: {}",
            &serialized_proposition
        );

        if let Err(e) = self
            .redis_connection.borrow_mut()
            .rpush::<_, _, bool>(queue_name, &serialized_proposition)
        {
            trace!("Storage::add_to_training_queue - Error adding proposition to training queue in Redis: {}", e);
            return Err(Box::new(e));
        }

        trace!("Storage::add_to_training_queue - Proposition added to training queue successfully");

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

    fn get_propositions_from_queue(
        &mut self,
        queue_name: &String,
    ) -> Result<Vec<Proposition>, Box<dyn Error>> {
        trace!(
            "Storage::get_propositions_from_queue - Start. Queue name: {}",
            queue_name
        );

        let mut propositions = Vec::new();

        // Attempt to pop one element at a time from the Redis queue
        while let Some(serialized_proposition) = self
            .redis_connection.borrow_mut()
            .lpop::<_, Option<String>>(queue_name, None)?
        {
            match serde_json::from_str(&serialized_proposition)
                .map_err(|e| Box::new(e) as Box<dyn Error>)
            {
                Ok(proposition) => propositions.push(proposition),
                Err(e) => {
                    trace!("Storage::get_propositions_from_queue - Error deserializing proposition: {}", e);
                    return Err(e);
                }
            }
        }

        trace!("Storage::get_propositions_from_queue - Retrieved and deserialized propositions successfully");

        Ok(propositions)
    }

    pub fn get_training_questions(&mut self) -> Result<Vec<Proposition>, Box<dyn Error>> {
        let training_queue_name = String::from("training_queue");
        self.get_propositions_from_queue(&training_queue_name)
    }

    pub fn get_test_questions(&mut self) -> Result<Vec<Proposition>, Box<dyn Error>> {
        let test_queue_name = String::from("test_queue");
        self.get_propositions_from_queue(&test_queue_name)
    }
}

impl PropositionProbability for Storage {
    // Return Some if the probability exists in the table, or else None.
    fn get_proposition_probability(
        &mut self,
        proposition: &Proposition,
    ) -> Result<Option<f64>, Box<dyn Error>> {
        let search_string = proposition.search_string();

        // Use a match statement to handle the different outcomes
        match self
            .redis_connection.borrow_mut()
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
