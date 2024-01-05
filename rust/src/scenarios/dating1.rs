use crate::model::{
    config::CONFIG,
    creators::{
        conjunction, constant, implication, object, proposition, relation, subject, variable,
    },
    objects::{Domain, Entity, RoleMap, Proposition},
    storage::Storage,
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

pub fn setup_train(storage: &mut Storage) -> Result<(), Box<dyn Error>> {
    storage.drop_all_dbs().map_err(|e| e.to_string())?;

    let config = CONFIG.get().expect("Config not initialized");
    let total_members_each_class = config.entities_per_domain;
    let entity_domains = [Domain::Jack, Domain::Jill];

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
    // Retrieve entities in the Jill domain
    let jill_domain = Domain::Jill.to_string(); // Convert enum to string and make lowercase
    let jills = storage.get_entities_in_domain(&jill_domain)?;
    trace!("Initial number of jills: {}", jills.len());

    let exciting = constant(Domain::Verb, "exciting".to_string());
    let lonely = constant(Domain::Verb, "lonely".to_string());
    let like = constant(Domain::Verb, "like".to_string());
    let date = constant(Domain::Verb, "date".to_string());

    for i in 0..jacks.len() {
        let jack_entity = &jacks[i];
        let jill_entity = &jills[i];

        let p_jack_lonely = cointoss();
        let p_jill_exciting: f64 = cointoss();
        let p_jill_likes_jack: f64 = cointoss();
        let p_jack_likes_jill = numeric_or(p_jack_lonely, p_jill_exciting);

        {
            trace!("Jack entity part 2: {:?}", jack_entity);
            let jack = constant(jack_entity.domain, jack_entity.name.clone());
            let jack_lonely = proposition(vec![subject(jack), relation(lonely.clone())]);
    
            trace!(
                "Jack Lonely: {:?}, Probability: {}",
                jack_lonely,
                p_jack_lonely
            ); // Logging
    
            // Assuming `store_proposition` is a method in your Storage struct
            storage.store_proposition(&jack_lonely, p_jack_lonely)?;
        }

        {
            let jill = constant(jill_entity.domain, jill_entity.name.clone());
            let jill_exciting = proposition(vec![subject(jill), relation(exciting.clone())]);
    
            trace!(
                "Jill Exciting: {:?}, Probability: {}",
                jill_exciting,
                p_jill_exciting
            ); // Logging
    
            // Assuming `store_proposition` is a method in your Storage struct
            storage.store_proposition(&jill_exciting, p_jill_exciting)?;
        }

        {
            let jill = constant(jill_entity.domain, jill_entity.name.clone());
            let jack = constant(jack_entity.domain, jack_entity.name.clone());

            // "likes(jill, jack)"
            let jill_likes_jack = proposition(vec![
                subject(jill.clone()),
                relation(like.clone()),
                object(jack.clone()),
            ]);
            trace!(
                "Jill likes Jack: {:?}, Probability: {}",
                jill_likes_jack,
                p_jill_likes_jack
            ); // Logging
            storage.store_proposition(&jill_likes_jack, p_jill_likes_jack)?;
        }

        {
            let jill = constant(jill_entity.domain, jill_entity.name.clone());
            let jack = constant(jack_entity.domain, jack_entity.name.clone());

            let jack_likes_jill = proposition(vec![
                subject(jack.clone()),
                relation(like.clone()),
                object(jill.clone()),
            ]);
            trace!(
                "Jack likes Jill: {:?}, Probability: {}",
                jack_likes_jill,
                p_jack_likes_jill
            ); // Logging
            storage.store_proposition(&jack_likes_jill, p_jack_likes_jill)?;
        }
        {
            let jill = constant(jill_entity.domain, jill_entity.name.clone());
            let jack = constant(jack_entity.domain, jack_entity.name.clone());

            // "dates(jack, jill)" based on "likes(jack, jill) and likes(jill, jack)"
            let jack_dates_jill =
                proposition(vec![subject(jack), relation(date.clone()), object(jill)]);
            let p_jack_dates_jill = numeric_and(p_jack_likes_jill, p_jill_likes_jack);
            trace!(
                "Jack dates Jill: {:?}, Probability: {}",
                jack_dates_jill,
                p_jack_dates_jill
            ); // Logging
            storage.store_proposition(&jack_dates_jill, p_jack_dates_jill)?;
        }

    }

    let xjack = variable(Domain::Jack);
    let xjill = variable(Domain::Jill);

    let implications = vec![
        // if jack is lonely, he will date any jill
        implication(
            conjunction(vec![proposition(vec![
                subject(xjack.clone()),
                relation(lonely.clone()),
            ])]),
            proposition(vec![
                subject(xjack.clone()),
                relation(like.clone()),
                object(xjill.clone()),
            ]),
            vec![RoleMap::new(HashMap::from([(
                "subject".to_string(),
                "subject".to_string(),
            )]))],
        ),
        // if jill is exciting, any jack will date her
        implication(
            conjunction(vec![proposition(vec![
                subject(xjill.clone()),
                relation(exciting.clone()),
            ])]),
            proposition(vec![
                subject(xjack.clone()),
                relation(like.clone()),
                object(xjill.clone()),
            ]),
            vec![RoleMap::new(HashMap::from([(
                "object".to_string(),
                "subject".to_string(),
            )]))],
        ),
        // if jill likes jack, then jack dates jill
        implication(
            conjunction(vec![proposition(vec![
                subject(xjill.clone()),
                relation(like.clone()),
                object(xjack.clone()),
            ])]),
            proposition(vec![
                subject(xjack.clone()),
                relation(date.clone()),
                object(xjill.clone()),
            ]),
            vec![RoleMap::new(HashMap::from([
                ("subject".to_string(), "object".to_string()),
                ("object".to_string(), "subject".to_string()),
            ]))],
        ),
        // if jack likes jill, then jack dates jill
        implication(
            conjunction(vec![proposition(vec![
                subject(xjack.clone()),
                relation(like.clone()),
                object(xjill.clone()),
            ])]),
            proposition(vec![subject(xjack), relation(date.clone()), object(xjill)]), // clone `date` here
            vec![RoleMap::new(HashMap::from([
                ("subject".to_string(), "subject".to_string()),
                ("object".to_string(), "object".to_string()),
            ]))],
        ),
    ];

    for implication in implications.iter() {
        trace!("Storing implication: {:?}", implication); // Logging, replace with your actual logger if necessary

        // Assuming `store_implication` is a method of your Storage struct
        storage.store_implication(implication)?;
    }

    // Additional functions
    fn numeric_or(a: f64, b: f64) -> f64 {
        f64::min(a + b, 1.0)
    }

    fn numeric_and(a: f64, b: f64) -> f64 {
        a * b
    }

    Ok(())
}

// Returns a vector of "test propositions".
pub fn setup_test(storage: &mut Storage) -> Result<Vec<Proposition>, Box<dyn Error>> {
    let total_members_each_class = 10; // test size
    // Retrieve entities in the Jack domain
    let jack_domain = Domain::Jack.to_string(); // Convert enum to string and make lowercase
    let jacks: Vec<Entity> = storage.get_entities_in_domain(&jack_domain)?;
    trace!("Initial number of jacks: {}", jacks.len());
    // Retrieve entities in the Jill domain
    let jill_domain = Domain::Jill.to_string(); // Convert enum to string and make lowercase
    let jills = storage.get_entities_in_domain(&jill_domain)?;
    trace!("Initial number of jills: {}", jills.len());

    let exciting = constant(Domain::Verb, "exciting".to_string());
    let lonely = constant(Domain::Verb, "lonely".to_string());
    let like = constant(Domain::Verb, "like".to_string());
    let date = constant(Domain::Verb, "date".to_string());

    let mut result = vec![];
    for i in 0..total_members_each_class {
        let jack_name = format!("test_{:?}{}", Domain::Jack, i); // Using Debug formatting for Domain enum
        let jack_entity = Entity {
            domain: Domain::Jack.clone(),
            name: jack_name.clone(),
        };
        let jill_name = format!("test_{:?}{}", Domain::Jill, i); // Using Debug formatting for Domain enum
        let jill_entity = Entity {
            domain: Domain::Jill.clone(),
            name: jill_name.clone(),
        };
        storage.store_entity(&jack_entity)?;
        storage.store_entity(&jill_entity)?;


        let p_jack_lonely = cointoss();
        let p_jill_exciting: f64 = cointoss();
        let p_jill_likes_jack: f64 = cointoss();

        {
            trace!("Jack entity part 2: {:?}", jack_entity);
            let jack = constant(jack_entity.domain, jack_entity.name.clone());
            let jack_lonely = proposition(vec![subject(jack), relation(lonely.clone())]);
    
            trace!(
                "Jack Lonely: {:?}, Probability: {}",
                jack_lonely,
                p_jack_lonely
            ); // Logging
    
            // Assuming `store_proposition` is a method in your Storage struct
            storage.store_proposition(&jack_lonely, p_jack_lonely)?;
        }

        {
            let jill = constant(jill_entity.domain, jill_entity.name.clone());
            let jill_exciting = proposition(vec![subject(jill), relation(exciting.clone())]);
    
            trace!(
                "Jill Exciting: {:?}, Probability: {}",
                jill_exciting,
                p_jill_exciting
            ); // Logging
    
            // Assuming `store_proposition` is a method in your Storage struct
            storage.store_proposition(&jill_exciting, p_jill_exciting)?;
        }

        {
            let jill = constant(jill_entity.domain, jill_entity.name.clone());
            let jack = constant(jack_entity.domain, jack_entity.name.clone());

            // "likes(jill, jack)"
            let jill_likes_jack = proposition(vec![
                subject(jill.clone()),
                relation(like.clone()),
                object(jack.clone()),
            ]);
            trace!(
                "Jill likes Jack: {:?}, Probability: {}",
                jill_likes_jack,
                p_jill_likes_jack
            ); // Logging
            storage.store_proposition(&jill_likes_jack, p_jill_likes_jack)?;
        }

        {
            let jill = constant(jill_entity.domain, jill_entity.name.clone());
            let jack = constant(jack_entity.domain, jack_entity.name.clone());

            // "dates(jack, jill)" based on "likes(jack, jill) and likes(jill, jack)"
            let jack_dates_jill =
                proposition(vec![subject(jack), relation(date.clone()), object(jill)]);
            result.push(jack_dates_jill);
        }
    }

    Ok(result)
}