use std::error::Error;

use crate::model::config::ConfigurationOptions;

use super::redis::RedisManager;

pub struct FactoryResources {
    pub config: ConfigurationOptions,
    pub redis:RedisManager,
}

impl FactoryResources {
    pub fn new(options: &ConfigurationOptions) -> Result<FactoryResources, Box<dyn Error>> {
        todo!()
    }
}