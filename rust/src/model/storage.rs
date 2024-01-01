use redis::Commands;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Entity {
    pub domain: String,
    pub name: String,
}

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
        con.sadd(&entity.domain, &entity.name)?;
        Ok(())
    }

    // Get entities in a domain
    pub fn get_entities_in_domain(&self, domain: &str) -> redis::RedisResult<Vec<Entity>> {
        let mut con = self.redis_client.get_connection()?;
        let names: Vec<String> = con.smembers(domain)?;
        Ok(names.into_iter().map(|name| Entity {
            domain: domain.to_string(),
            name,
        }).collect())
    }
}
