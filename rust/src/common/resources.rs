use std::error::Error;
use super::{redis::RedisManager, setup::CommandLineOptions};

pub struct FactoryResources {
    pub config: CommandLineOptions,
    pub redis: RedisManager,
}

impl FactoryResources {
    pub fn new(options: &CommandLineOptions) -> Result<FactoryResources, Box<dyn Error>> {
        Ok(FactoryResources {
            config: options.clone(),
            redis: RedisManager::new()?,
        })
    }
}
