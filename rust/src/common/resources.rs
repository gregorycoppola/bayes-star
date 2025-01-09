use std::{error::Error, sync::{Arc, Mutex}};
use super::{redis::RedisManager, setup::CommandLineOptions};

pub struct NamespaceBundle {
    pub namespace: String,
    pub connection: Arc<Mutex<redis::Connection>>,
}

impl NamespaceBundle {
    pub fn new_from_cli(options: &CommandLineOptions) -> Result<NamespaceBundle, Box<dyn Error>> {
        let namespace = options.scenario_name.clone();
        let manager = RedisManager::new()?;
        let connection = manager.get_arc_mutex_guarded_connection()?;
        Ok(NamespaceBundle {
            namespace,
            connection,
        })
    }
}