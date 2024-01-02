use std::error::Error;

use crate::model::{
    objects::{Domain, Entity},
    storage::Storage, creators::constant,
};

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

    // Retrieve entities in the Jack domain
    let jack_domain = Domain::Jack.to_string().to_lowercase(); // Convert enum to string and make lowercase
    let jacks = storage.get_entities_in_domain(&jack_domain)?;
    for jack in jacks {
        println!("Jack entity: {:?}", jack);
    }

    // Retrieve entities in the Jill domain
    let jill_domain = Domain::Jill.to_string().to_lowercase(); // Convert enum to string and make lowercase
    let jills = storage.get_entities_in_domain(&jill_domain)?;
    for jill in jills {
        println!("Jill entity: {:?}", jill);
    }

    let exciting = constant(Domain::Verb, "exciting".to_string());
    let lonely = constant(Domain::Verb, "lonely".to_string());
    let like = constant(Domain::Verb, "like".to_string());
    let date = constant(Domain::Verb, "date".to_string());
    
    Ok(())
}
