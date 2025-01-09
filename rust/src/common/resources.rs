use std::{error::Error, sync::{Arc, Mutex}};
use super::{redis::RedisManager, setup::CommandLineOptions};

pub struct ResourceBundle {
    pub connection: Arc<Mutex<redis::Connection>>,
}

impl ResourceBundle {
    pub fn new(options: &CommandLineOptions) -> Result<ResourceBundle, Box<dyn Error>> {
        let manager = RedisManager::new()?;
        let connection = manager.get_arc_mutex_guarded_connection()?;
        Ok(ResourceBundle {
            connection,
        })
    }
}