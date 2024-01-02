use crate::model::{
    creators::{constant, implication, object, predicate, relation, subject, variable},
    objects::{Domain, Entity, RoleMap},
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
    let jack_domain = Domain::Jack.to_string(); // Convert enum to string and make lowercase
    let jacks: Vec<Entity> = storage.get_entities_in_domain(&jack_domain)?;
    println!("Initial number of jacks: {}", jacks.len());
    for jack in jacks.clone() {
        println!("Jack entity: {:?}", jack);
    }

    // Retrieve entities in the Jill domain
    let jill_domain = Domain::Jill.to_string(); // Convert enum to string and make lowercase
    let jills = storage.get_entities_in_domain(&jill_domain)?;
    for jill in jills.clone() {
        println!("Jill entity: {:?}", jill);
    }

    let exciting = constant(Domain::Verb, "exciting".to_string());
    let lonely = constant(Domain::Verb, "lonely".to_string());
    let like = constant(Domain::Verb, "like".to_string());
    let date = constant(Domain::Verb, "date".to_string());

    let mut independent_fact_map: HashMap<String, f64> = HashMap::new();

    println!("Number of jacks before second loop: {}", jacks.len());
    for jack_entity in jacks.clone() {
        println!("Jack entity part 2: {:?}", jack_entity);

        let jack = constant(jack_entity.domain, jack_entity.name.clone());
        let jack_lonely = predicate(vec![subject(jack), relation(lonely.clone())]);
        let p_jack_lonely = cointoss();

        println!(
            "Jack Lonely: {:?}, Probability: {}",
            jack_lonely, p_jack_lonely
        ); // Logging

        // Assuming `store_proposition` is a method in your Storage struct
        storage.store_proposition(&jack_lonely, p_jack_lonely)?;

        // Inserting into the independent fact map
        independent_fact_map.insert(format!("{:?}", jack_lonely), p_jack_lonely);
    }

    for jill_entity in &jills {
        let jill = constant(jill_entity.domain, jill_entity.name.clone());
        let jill_exciting = predicate(vec![subject(jill), relation(exciting.clone())]);
        let p_jill_exciting = cointoss();

        println!(
            "Jill Exciting: {:?}, Probability: {}",
            jill_exciting, p_jill_exciting
        ); // Logging

        // Assuming `store_proposition` is a method in your Storage struct
        storage.store_proposition(&jill_exciting, p_jill_exciting)?;

        // Inserting into the independent fact map
        independent_fact_map.insert(format!("{:?}", jill_exciting), p_jill_exciting);
    }

    // Assumed imports and setup...

    for jack_entity in jacks.iter() {
        for jill_entity in jills.iter() {
            let jill = constant(jill_entity.domain, jill_entity.name.clone());
            let jack = constant(jack_entity.domain, jack_entity.name.clone());

            // "likes(jill, jack)"
            let jill_likes_jack = predicate(vec![
                subject(jill.clone()),
                relation(like.clone()),
                object(jack.clone()),
            ]);
            let p_jill_likes_jack = cointoss();
            println!(
                "Jill likes Jack: {:?}, Probability: {}",
                jill_likes_jack, p_jill_likes_jack
            ); // Logging
            storage.store_proposition(&jill_likes_jack, p_jill_likes_jack)?;
            independent_fact_map.insert(format!("{:?}", jill_likes_jack), p_jill_likes_jack);

            // "likes(jack, jill)" based on "lonely(jack) or exciting(jill)"
            let jack_lonely = predicate(vec![subject(jack.clone()), relation(lonely.clone())]);
            let p_jack_lonely = *independent_fact_map
                .get(&format!("{:?}", jack_lonely))
                .unwrap_or(&0.0);
            let jill_exciting = predicate(vec![subject(jill.clone()), relation(exciting.clone())]);
            let p_jill_exciting = *independent_fact_map
                .get(&format!("{:?}", jill_exciting))
                .unwrap_or(&0.0);
            let jack_likes_jill = predicate(vec![
                subject(jack.clone()),
                relation(like.clone()),
                object(jill.clone()),
            ]);
            let p_jack_likes_jill = numeric_or(p_jack_lonely, p_jill_exciting);
            println!(
                "Jack likes Jill: {:?}, Probability: {}",
                jack_likes_jill, p_jack_likes_jill
            ); // Logging
            storage.store_proposition(&jack_likes_jill, p_jack_likes_jill)?;
            independent_fact_map.insert(format!("{:?}", jack_likes_jill), p_jack_likes_jill);

            // "dates(jack, jill)" based on "likes(jack, jill) and likes(jill, jack)"
            let jack_dates_jill =
                predicate(vec![subject(jack), relation(date.clone()), object(jill)]);
            let p_jack_dates_jill = numeric_and(p_jack_likes_jill, p_jill_likes_jack);
            println!(
                "Jack dates Jill: {:?}, Probability: {}",
                jack_dates_jill, p_jack_dates_jill
            ); // Logging
            storage.store_proposition(&jack_dates_jill, p_jack_dates_jill)?;
        }
    }

    let xjack = variable(Domain::Jack);
    let xjill = variable(Domain::Jill);

    // ...

    let implications = vec![
        // if jack is lonely, he will date any jill
        implication(
            predicate(vec![subject(xjack.clone()), relation(lonely.clone())]),
            predicate(vec![
                subject(xjack.clone()),
                relation(like.clone()),
                object(xjill.clone()),
            ]),
            RoleMap::new(HashMap::from([(
                "subject".to_string(),
                "subject".to_string(),
            )])),
        ),
        // if jill is exciting, any jack will date her
        implication(
            predicate(vec![subject(xjill.clone()), relation(exciting.clone())]),
            predicate(vec![
                subject(xjack.clone()),
                relation(like.clone()),
                object(xjill.clone()),
            ]),
            RoleMap::new(HashMap::from([(
                "object".to_string(),
                "subject".to_string(),
            )])),
        ),
        // if jill likes jack, then jack dates jill
        implication(
            predicate(vec![
                subject(xjill.clone()),
                relation(like.clone()),
                object(xjack.clone()),
            ]),
            predicate(vec![
                subject(xjack.clone()),
                relation(date.clone()),
                object(xjill.clone()),
            ]),
            RoleMap::new(HashMap::from([
                ("subject".to_string(), "object".to_string()),
                ("object".to_string(), "subject".to_string()),
            ])),
        ),
        // if jack likes jill, then jack dates jill
        implication(
            predicate(vec![
                subject(xjack.clone()),
                relation(like.clone()),
                object(xjill.clone()),
            ]),
            predicate(vec![subject(xjack), relation(date.clone()), object(xjill)]), // clone `date` here
            RoleMap::new(HashMap::from([
                ("subject".to_string(), "subject".to_string()),
                ("object".to_string(), "object".to_string()),
            ])),
        ),
    ];

    for implication in implications.iter() {
        println!("Storing implication: {:?}", implication); // Logging, replace with your actual logger if necessary
    
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
