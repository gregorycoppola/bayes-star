use std::{error::Error, sync::{Arc, Mutex}};
use super::{redis::RedisManager, setup::CommandLineOptions};

pub struct ResourceContext {
    pub connection: Arc<Mutex<redis::Connection>>,
}

impl ResourceContext {
    pub fn new(options: &CommandLineOptions) -> Result<ResourceContext, Box<dyn Error>> {
        let manager = RedisManager::new()?;
        let connection = manager.get_arc_mutex_guarded_connection()?;
        Ok(ResourceContext {
            connection,
        })
    }
}