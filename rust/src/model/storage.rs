use crate::model::objects::{Domain, Entity, Implication, Proposition};
use redis::{Commands, Connection};
use std::error::Error;

pub struct Storage {
    redis_connection: redis::Connection,
}


impl Drop for Storage {
    fn drop(&mut self) {
        // The Drop trait for Arc<Client> will automatically be called here,
        // reducing the reference count. If this Storage instance holds the last 
        // reference to the client, the client will be dropped and its resources
        // (like network connections) will be cleaned up.
    }
}

pub trait PropositionProbability {
    fn get_proposition_probability(&mut self, proposition: &Proposition) -> Result<Option<f64>, Box<dyn Error>>;
}

impl Storage {
    // Initialize new Storage with a Redis connection
    pub fn new(connection: Connection) -> Result<Self, redis::RedisError> {
        Ok(Storage { redis_connection: connection })
    }
    pub fn drop_all_dbs(&mut self) -> Result<(), Box<dyn Error>> {
        redis::cmd("FLUSHDB").query(&mut self.redis_connection)?;
        trace!("Database flushed successfully");

        Ok(())
    }

    pub fn get_redis_connection(&mut self) -> &mut redis::Connection {
        &mut self.redis_connection
    }
    // Store an entity
    pub fn store_entity(&mut self, entity: &Entity) -> Result<(), Box<dyn Error>> {
        trace!("Storing entity in domain '{}': {}", entity.domain, entity.name); // Logging
        self.redis_connection.sadd(&entity.domain.to_string(), &entity.name)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(())
    }

    pub fn get_entities_in_domain(&mut self, domain: &str) -> Result<Vec<Entity>, Box<dyn Error>> {
        trace!("Getting entities in domain '{}'", domain.clone()); // Logging

        let names: Vec<String> = self.redis_connection
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
        trace!("Storage::store_proposition - Start. Input proposition: {:?}, probability: {}", proposition, probability);

        let search_string = proposition.search_string();
        trace!("Storage::store_proposition - Computed search_string: {}", search_string);
    
        let record = match serde_json::to_string(proposition) {
            Ok(record) => record,
            Err(e) => {
                trace!("Storage::store_proposition - Error serializing proposition: {}", e);
                return Err(Box::new(e));
            }
        };
        trace!("Storage::store_proposition - Serialized proposition record: {} {}", &search_string, &record);
    
        if let Err(e) = self.redis_connection.hset::<_, _, _, bool>("propositions", &search_string, &record) {
            trace!("Storage::store_proposition - Error storing proposition in Redis: {}", e);
            return Err(Box::new(e));
        }
    
        match self.store_proposition_probability(proposition, probability) {
            Ok(_) => trace!("Storage::store_proposition - Completed successfully"),
            Err(e) => trace!("Storage::store_proposition - Error in store_proposition_probability: {}", e),
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
        trace!("Storage::store_proposition_probability - Computed search_string: {}", search_string);
    
        if let Err(e) = self.redis_connection.hset::<&str, &str, String, bool>("probs", &search_string, probability.to_string()) {
            trace!("Storage::store_proposition_probability - Error storing probability in Redis: {}", e);
            return Err(Box::new(e));
        }
        
    
        trace!("Storage::store_proposition_probability - Completed successfully");
        Ok(())
    }
    

    // Get all propositions
    pub fn get_all_propositions(&mut self) -> Result<Vec<Proposition>, Box<dyn Error>> {
        trace!("Storage::get_all_propositions - Retrieving all propositions");
        let all_values: std::collections::HashMap<String, String> = self.redis_connection.hgetall("propositions").map_err(|e| Box::new(e) as Box<dyn Error>)?;

        all_values.into_iter().map(|(key, value)| {
            trace!("Storage::get_all_propositions - Key: {}, Value: {}", key, value);
            serde_json::from_str(&value).map_err(|e| Box::new(e) as Box<dyn Error>)
        }).collect()
    }

    pub fn store_implication(&mut self, implication: &Implication) -> Result<(), Box<dyn Error>> {
        let record =
            serde_json::to_string(implication).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        self.redis_connection.sadd("implications", &record)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        self.store_links(implication)
    }

    pub fn store_links(&mut self, implication: &Implication) -> Result<(), Box<dyn Error>> {
        let search_string = implication.search_string();
        let record =
            serde_json::to_string(implication).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        self.redis_connection.sadd(&search_string, &record)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(())
    }

    // Get all Implications
    pub fn get_all_implications(&mut self) -> Result<Vec<Implication>, Box<dyn Error>> {
        let all_values: Vec<String> = self.redis_connection
            .smembers("implications")
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        all_values
            .into_iter()
            .map(|record| serde_json::from_str(&record).map_err(|e| Box::new(e) as Box<dyn Error>))
            .collect()
    }

    pub fn find_premises(&mut self, search_string: &str) -> Result<Vec<Implication>, Box<dyn Error>> {
        trace!("find_premises: {:?}", &search_string);
        let set_members: Vec<String> = self.redis_connection
            .smembers(search_string)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        set_members
            .into_iter()
            .map(|record| serde_json::from_str(&record).map_err(|e| Box::new(e) as Box<dyn Error>))
            .collect()
    }

    pub fn maybe_add_to_training(
        &mut self,
        is_training: bool,
        proposition: &Proposition,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }    
    pub fn maybe_add_to_test(
        &mut self,
        is_test: bool,
        proposition: &Proposition,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    pub fn get_test_questions(
        &mut self,
    ) -> Result<Vec<Proposition>, Box<dyn Error>> {
        todo!()
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
        match self.redis_connection.hget::<_, _, String>("probs", &search_string) {
            Ok(probability_str) => {
                // Found the entry, parse it
                let probability = probability_str.parse::<f64>()
                    .map_err(|e| Box::new(e) as Box<dyn Error>)?;
                Ok(Some(probability))
            },
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