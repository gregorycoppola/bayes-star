use crate::model::objects::{Domain, Entity, Implication, Proposition};
use redis::{Client, Commands};
use std::{error::Error, sync::Arc};

pub struct Storage {
    redis_client: Arc<redis::Client>,
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
    // Initialize new Storage with a Redis client
    pub fn new(redis_client: Arc<redis::Client>) -> Self {
        Storage { redis_client }
    }
    pub fn drop_all_dbs(&self) -> Result<(), Box<dyn Error>> {
        let mut conn = self.redis_client.get_connection()?;
        redis::cmd("FLUSHDB").query(&mut conn)?;
        println!("Database flushed successfully");

        Ok(())
    }

    pub fn get_redis_client(&self) -> &Arc<Client> {
        &self.redis_client
    }

    // Store an entity
    pub fn store_entity(&self, entity: &Entity) -> Result<(), Box<dyn Error>> {
        println!("Storing entity in domain '{}': {}", entity.domain, entity.name); // Logging
        let mut con = self
            .redis_client
            .get_connection()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        con.sadd(&entity.domain.to_string(), &entity.name)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(())
    }

    pub fn get_entities_in_domain(&self, domain: &str) -> Result<Vec<Entity>, Box<dyn Error>> {
        println!("Getting entities in domain '{}'", domain.clone()); // Logging

        let mut con = self
            .redis_client
            .get_connection()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        let names: Vec<String> = con
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
        &self,
        proposition: &Proposition,
        probability: f64,
    ) -> Result<(), Box<dyn Error>> {
        println!("Storage::store_proposition - Start. Input proposition: {:?}, probability: {}", proposition, probability);
    
        let mut con = match self.redis_client.get_connection() {
            Ok(con) => con,
            Err(e) => {
                println!("Storage::store_proposition - Error getting Redis connection: {}", e);
                return Err(Box::new(e));
            }
        };
        let search_string = proposition.search_string();
        println!("Storage::store_proposition - Computed search_string: {}", search_string);
    
        let record = match serde_json::to_string(proposition) {
            Ok(record) => record,
            Err(e) => {
                println!("Storage::store_proposition - Error serializing proposition: {}", e);
                return Err(Box::new(e));
            }
        };
        println!("Storage::store_proposition - Serialized proposition record: {} {}", &search_string, &record);
    
        if let Err(e) = con.hset::<_, _, _, bool>("propositions", &search_string, &record) {
            println!("Storage::store_proposition - Error storing proposition in Redis: {}", e);
            return Err(Box::new(e));
        }
    
        match self.store_proposition_probability(proposition, probability) {
            Ok(_) => println!("Storage::store_proposition - Completed successfully"),
            Err(e) => println!("Storage::store_proposition - Error in store_proposition_probability: {}", e),
        }
    
        Ok(())
    }

    pub fn store_proposition_probability(
        &self,
        proposition: &Proposition,
        probability: f64,
    ) -> Result<(), Box<dyn Error>> {
        println!("Storage::store_proposition_probability - Start. Input proposition: {:?}, probability: {}", proposition, probability);
    
        let mut con = match self.redis_client.get_connection() {
            Ok(con) => con,
            Err(e) => {
                println!("Storage::store_proposition_probability - Error getting Redis connection: {}", e);
                return Err(Box::new(e));
            }
        };
        let search_string = proposition.search_string();
        println!("Storage::store_proposition_probability - Computed search_string: {}", search_string);
    
        if let Err(e) = con.hset::<&str, &str, String, bool>("probs", &search_string, probability.to_string()) {
            println!("Storage::store_proposition_probability - Error storing probability in Redis: {}", e);
            return Err(Box::new(e));
        }
        
    
        println!("Storage::store_proposition_probability - Completed successfully");
        Ok(())
    }
    

    // Get all propositions
    pub fn get_all_propositions(&self) -> Result<Vec<Proposition>, Box<dyn Error>> {
        println!("Storage::get_all_propositions - Retrieving all propositions");

        let mut con = self.redis_client.get_connection().map_err(|e| Box::new(e) as Box<dyn Error>)?;
        let all_values: std::collections::HashMap<String, String> = con.hgetall("propositions").map_err(|e| Box::new(e) as Box<dyn Error>)?;

        all_values.into_iter().map(|(key, value)| {
            println!("Storage::get_all_propositions - Key: {}, Value: {}", key, value);
            serde_json::from_str(&value).map_err(|e| Box::new(e) as Box<dyn Error>)
        }).collect()
    }

    // Get the probability of a proposition
    pub fn get_proposition_probability(
        &self,
        proposition: &Proposition,
    ) -> Result<f64, Box<dyn Error>> {
        let mut con = self
            .redis_client
            .get_connection()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let search_string = proposition.search_string();
        let probability_str: String = con
            .hget("probs", &search_string)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let probability = probability_str
            .parse::<f64>()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(probability)
    }

    pub fn store_implication(&self, implication: &Implication) -> Result<(), Box<dyn Error>> {
        let mut con = self
            .redis_client
            .get_connection()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let search_string = implication.search_string();
        let record =
            serde_json::to_string(implication).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        con.sadd("implications", &record)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        self.store_links(implication)
    }

    pub fn store_links(&self, implication: &Implication) -> Result<(), Box<dyn Error>> {
        let mut con = self
            .redis_client
            .get_connection()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let search_string = implication.search_string();
        let record =
            serde_json::to_string(implication).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        con.sadd(&search_string, &record)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(())
    }

    // Get all Implications
    pub fn get_all_implications(&self) -> Result<Vec<Implication>, Box<dyn Error>> {
        let mut con = self
            .redis_client
            .get_connection()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let all_values: Vec<String> = con
            .smembers("implications")
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        all_values
            .into_iter()
            .map(|record| serde_json::from_str(&record).map_err(|e| Box::new(e) as Box<dyn Error>))
            .collect()
    }

    pub fn find_premises(&self, search_string: &str) -> Result<Vec<Implication>, Box<dyn Error>> {
        let mut con = self
            .redis_client
            .get_connection()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let set_members: Vec<String> = con
            .smembers(search_string)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        set_members
            .into_iter()
            .map(|record| serde_json::from_str(&record).map_err(|e| Box::new(e) as Box<dyn Error>))
            .collect()
    }
}
