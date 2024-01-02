use crate::model::objects::{Domain, Entity, Proposition, Implication};
use redis::Commands;
use std::{error::Error, sync::Arc};

pub struct Storage {
    redis_client: Arc<redis::Client>,
}

impl Storage {
    // Initialize new Storage with a Redis client
    pub fn new(redis_client: Arc<redis::Client>) -> Self {
        Storage { redis_client }
    }

    // Store an entity
    pub fn store_entity(&self, entity: &Entity) -> Result<(), Box<dyn Error>> {
        let mut con = self
            .redis_client
            .get_connection()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        con.sadd(&entity.domain.to_string(), &entity.name)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(())
    }

    // Get entities in a domain
    pub fn get_entities_in_domain(&self, domain: &str) -> Result<Vec<Entity>, Box<dyn Error>> {
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
                domain: Domain::from_str(&name).expect("Domain not recognized."), // Adjust this based on your Domain handling
                name,
            })
            .collect())
    }

    pub fn store_proposition(
        &self,
        proposition: &Proposition,
        probability: f64,
    ) -> Result<(), Box<dyn Error>> {
        let mut con = self
            .redis_client
            .get_connection()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        let search_string = proposition.search_string();
        let record =
            serde_json::to_string(proposition).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        con.hset("propositions", &search_string, &record)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        self.store_proposition_probability(proposition, probability)
    }

    // Store the probability of a proposition
    fn store_proposition_probability(
        &self,
        proposition: &Proposition,
        probability: f64,
    ) -> Result<(), Box<dyn Error>> {
        let mut con = self
            .redis_client
            .get_connection()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        // Implement storing the probability
        // For example, using proposition's search string as a key
        // ...
        Ok(())
    }

    // Get all propositions
    pub fn get_all_propositions(&self) -> Result<Vec<Proposition>, Box<dyn Error>> {
        let mut con = self
            .redis_client
            .get_connection()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let all_values: std::collections::HashMap<String, String> = con
            .hgetall("propositions")
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        all_values
            .into_iter()
            .map(|(_, value)| {
                serde_json::from_str(&value).map_err(|e| Box::new(e) as Box<dyn Error>)
            })
            .collect()
    }

    // Get the probability of a proposition
    pub fn get_proposition_probability(&self, proposition: &Proposition) -> Result<f64, Box<dyn Error>> {
        let mut con = self.redis_client.get_connection()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let search_string = proposition.search_string();
        let probability_str: String = con.hget("probs", &search_string)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let probability = probability_str.parse::<f64>()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(probability)
    }

    pub fn store_implication(&self, implication: &Implication) -> Result<(), Box<dyn Error>> {
        let mut con = self.redis_client.get_connection()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        
        let search_string = implication.search_string();
        let record = serde_json::to_string(implication)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        con.sadd("implications", &record)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        self.store_links(implication)
    }

    // Store links for an Implication (placeholder function)
    fn store_links(&self, implication: &Implication) -> Result<(), Box<dyn Error>> {
        // Implement storing links associated with the implication
        // ...
        Ok(())
    }

    // Get all Implications
    pub fn get_all_implications(&self) -> Result<Vec<Implication>, Box<dyn Error>> {
        let mut con = self.redis_client.get_connection()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let all_values: Vec<String> = con.smembers("implications")
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        all_values.into_iter().map(|record| {
            serde_json::from_str(&record)
                .map_err(|e| Box::new(e) as Box<dyn Error>)
        }).collect()
    }
}
