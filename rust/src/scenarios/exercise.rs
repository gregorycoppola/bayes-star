use crate::model::{
    config::CONFIG,
    creators::{
        conjunction, constant, link, proposition, relation, subject, variable,
    },
    objects::{Domain, Entity, RoleMap, Proposition},
    storage::GraphicalModel,
};
use rand::Rng; // Import Rng trait
use std::{collections::HashMap, error::Error};

fn cointoss() -> f64 {
    let mut rng = rand::thread_rng(); // Get a random number generator
    if rng.gen::<f64>() < 0.5 {
        1.0
    } else {
        0.0
    }
}

fn weighted_cointoss(threshold: f64) -> f64 {
    let mut rng = rand::thread_rng(); // Get a random number generator
    if rng.gen::<f64>() < threshold {
        1.0
    } else {
        0.0
    }
}

pub fn setup_scenario(storage: &mut GraphicalModel) -> Result<(), Box<dyn Error>> {
    storage.drop_all_dbs().map_err(|e| e.to_string())?;

    let config = CONFIG.get().expect("Config not initialized");
    let total_members_each_class = config.entities_per_domain;
    let entity_domains = [Domain::Jack];

    for domain in entity_domains.iter() {
        for i in 0..total_members_each_class {
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
            trace!("Stored entity: {:?}", entity);
        }
    }

    // Retrieve entities in the Jack domain
    let jack_domain = Domain::Jack.to_string(); // Convert enum to string and make lowercase
    let jacks: Vec<Entity> = storage.get_entities_in_domain(&jack_domain)?;
    trace!("Initial number of jacks: {}", jacks.len());

    let educated = constant(Domain::Verb, "educated".to_string());
    let rich = constant(Domain::Verb, "rich".to_string());

    for i in 0..jacks.len() {
        let jack_entity = &jacks[i];

        let p_jack_educated = cointoss();
        let p_jack_rich =  if p_jack_educated > 0.5 {
            weighted_cointoss(0.8)
        } else {
            weighted_cointoss(0.2)
        };

        {
            trace!("Jack entity part 2: {:?}", jack_entity);
            let jack = constant(jack_entity.domain, jack_entity.name.clone());
            let jack_educated = proposition(vec![subject(jack), relation(educated.clone())]);

            trace!(
                "Jack educated: {:?}, Probability: {}",
                jack_educated.search_string(),
                p_jack_educated
            ); // Logging

            // Assuming `store_proposition` is a method in your GraphicalModel struct
            storage.store_proposition(&jack_educated, p_jack_educated)?;
        }
        {
            trace!("Jack entity part 2: {:?}", jack_entity);
            let jack = constant(jack_entity.domain, jack_entity.name.clone());
            let jack_rich = proposition(vec![subject(jack), relation(rich.clone())]);

            trace!(
                "Jack educated: {:?}, Probability: {}",
                jack_rich.search_string(),
                p_jack_rich
            ); // Logging

            // Assuming `store_proposition` is a method in your GraphicalModel struct
            storage.store_proposition(&jack_rich, p_jack_rich)?;
        }
    }

    let xjack = variable(Domain::Jack);
    let links = vec![
        // if jack is educated, he will date any jill
        link(
            conjunction(vec![proposition(vec![
                subject(xjack.clone()),
                relation(educated.clone()),
            ])]),
            proposition(vec![
                subject(xjack.clone()),
                relation(rich.clone()),
            ]),
            vec![RoleMap::new(HashMap::from([(
                "subject".to_string(),
                "subject".to_string(),
            )]))],
        ),
    ];

    for link in links.iter() {
        trace!("Storing link: {:?}", link); // Logging, replace with your actual logger if necessary
        // Assuming `store_link` is a method of your GraphicalModel struct
        storage.store_link(link)?;
    }

    Ok(())
}

// Returns a vector of "test propositions".
pub fn setup_test(storage: &mut GraphicalModel) -> Result<Vec<Proposition>, Box<dyn Error>> {
    let total_members_each_class = 10; // test size
    // Retrieve entities in the Jack domain
    let jack_domain = Domain::Jack.to_string(); // Convert enum to string and make lowercase
    let jacks: Vec<Entity> = storage.get_entities_in_domain(&jack_domain)?;
    trace!("Initial number of jacks: {}", jacks.len());
    // Retrieve entities in the Jill domain
    let jill_domain = Domain::Jill.to_string(); // Convert enum to string and make lowercase
    let jills = storage.get_entities_in_domain(&jill_domain)?;
    trace!("Initial number of jills: {}", jills.len());

    let entity_domains = [Domain::Jack];

    for domain in entity_domains.iter() {
        for i in 0..total_members_each_class {
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
            trace!("Stored entity: {:?}", entity);
        }
    }

    // Retrieve entities in the Jack domain
    let jack_domain = Domain::Jack.to_string(); // Convert enum to string and make lowercase
    let jacks: Vec<Entity> = storage.get_entities_in_domain(&jack_domain)?;
    trace!("Initial number of jacks: {}", jacks.len());

    let educated = constant(Domain::Verb, "educated".to_string());
    let rich = constant(Domain::Verb, "rich".to_string());

    let mut result = vec![];

    for i in 0..jacks.len() {
        let jack_entity = &jacks[i];

        let p_jack_educated = cointoss();
        let p_jack_rich =  if p_jack_educated > 0.5 {
            weighted_cointoss(0.8)
        } else {
            weighted_cointoss(0.2)
        };

        {
            trace!("Jack entity part 2: {:?}", jack_entity);
            let jack = constant(jack_entity.domain, jack_entity.name.clone());
            let jack_educated = proposition(vec![subject(jack), relation(educated.clone())]);

            trace!(
                "Jack educated: {:?}, Probability: {}",
                jack_educated.search_string(),
                p_jack_educated
            ); // Logging

            // Assuming `store_proposition` is a method in your GraphicalModel struct
            storage.store_proposition(&jack_educated, p_jack_educated)?;
        }
        {
            trace!("Jack entity part 2: {:?}", jack_entity);
            let jack = constant(jack_entity.domain, jack_entity.name.clone());
            let jack_rich = proposition(vec![subject(jack), relation(rich.clone())]);

            trace!(
                "Jack educated: {:?}, Probability: {}",
                jack_rich.search_string(),
                p_jack_rich
            ); // Logging

            // Assuming `store_proposition` is a method in your GraphicalModel struct
            storage.store_proposition(&jack_rich, p_jack_rich)?;
            result.push(jack_rich);
        }
    }

    Ok(result)
}