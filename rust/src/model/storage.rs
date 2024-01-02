use redis::Commands;
use std::sync::Arc;
use serde_json::{json, Value as JsonValue};
use crate::model::objects::{Domain, Entity, Proposition};
use serde::{Serialize, Deserialize};

pub struct Storage {
    redis_client: Arc<redis::Client>,
}

impl Storage {
    // Initialize new Storage with a Redis client
    pub fn new(redis_client: Arc<redis::Client>) -> Self {
        Storage { redis_client }
    }

    // Store an entity
    pub fn store_entity(&self, entity: &Entity) -> redis::RedisResult<()> {
        let mut con = self.redis_client.get_connection()?;
        con.sadd(&entity.domain.to_string(), &entity.name)?;
        Ok(())
    }

    // Get entities in a domain
    pub fn get_entities_in_domain(&self, domain: &str) -> redis::RedisResult<Vec<Entity>> {
        let mut con = self.redis_client.get_connection()?;
        let names: Vec<String> = con.smembers(domain)?;
        Ok(names.into_iter().map(|name| Entity {
            domain: Domain::from_str(&name).expect("Domain not recognized."),
            name,
        }).collect())
    }

    pub fn store_proposition(&self, proposition: &Proposition, probability: f64) -> redis::RedisResult<()> {
        let mut con = self.redis_client.get_connection()?;
        let search_string = proposition.search_string();
        let record = serde_json::to_string(proposition)?;
        con.hset("propositions", &search_string, &record)?;
        self.store_proposition_probability(proposition, probability)?;
        Ok(())
    }

    // Store the probability of a proposition
    fn store_proposition_probability(&self, proposition: &Proposition, probability: f64) -> redis::RedisResult<()> {
        let mut con = self.redis_client.get_connection()?;
        // Implement storing the probability
        // For example, using proposition's search string as a key
        // ...
        Ok(())
    }

    // Get all propositions
    pub fn get_all_propositions(&self) -> redis::RedisResult<Vec<Proposition>> {
        let mut con = self.redis_client.get_connection()?;
        let all_values: std::collections::HashMap<String, String> = con.hgetall("propositions")?;
        all_values.into_iter().map(|(_, value)| {
            serde_json::from_str(&value).expect("Invalid JSON for Proposition")
        }).collect()
    }
}
