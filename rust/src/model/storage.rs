use redis::Commands;
use std::sync::Arc;

use crate::model::objects::Domain;
use crate::model::objects::Entity;


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
}
