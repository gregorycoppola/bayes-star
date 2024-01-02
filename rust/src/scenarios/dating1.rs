use std::error::Error;

use crate::model::{storage::Storage, objects::{Domain, Entity}};

pub fn setup_scenario(storage: &mut Storage) -> Result<(), Box<dyn Error>> {
    storage.drop_all_dbs().map_err(|e| e.to_string())?;

    const TOTAL_MEMBERS_EACH_CLASS: usize = 32 * 2;
    let domains = [Domain::Jack, Domain::Jill];

    for domain in domains.iter() {
        for i in 0..TOTAL_MEMBERS_EACH_CLASS {
            let name = format!("{:?}{}", domain, i); // Using Debug formatting for Domain enum
            let entity = Entity {
                domain: domain.clone(),
                name: name.clone(),
            };

            // Assuming you have a `storage` instance of a struct that can store entities
            // and a `store_entity` method which handles storage.
            // Replace `storage.store_entity(entity)?;` with the actual method call
            storage.store_entity(&entity)?; 

            // Replace logger.noop() with your actual logging if needed
            println!("Stored entity: {:?}", entity);
        }
    }
    
    Ok(())
}
