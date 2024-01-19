use std::error::Error;
use super::{redis::RedisManager, setup::ConfigurationOptions};

pub struct FactoryResources {
    pub config: ConfigurationOptions,
    pub redis: RedisManager,
}

impl FactoryResources {
    pub fn new(options: &ConfigurationOptions) -> Result<FactoryResources, Box<dyn Error>> {
        Ok(FactoryResources {
            config: options.clone(),
            redis: RedisManager::new()?,
        })
    }
}
