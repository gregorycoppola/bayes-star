use crate::model::{
    config::CONFIG,
    creators::{
        conjunction, constant, implication, object, proposition, relation, subject, variable,
    },
    objects::{Domain, Entity, Proposition, RoleMap},
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

fn weighted_cointoss(threshold: f64) -> f64 {
    let mut rng = rand::thread_rng(); // Get a random number generator
    if rng.gen::<f64>() < threshold {
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

    for i in 0..total_members_each_class {
        let is_test = i % 10 == 9;
        let is_training = !is_test;
        let mut domain_entity_map:HashMap<String, Entity> = HashMap::new();
        for domain in entity_domains.iter() {
            let prefix = if is_test { "test" } else { "train" };
            let name = format!("{}_{:?}{}", &prefix, domain, i); // Using Debug formatting for Domain enum
            let entity = Entity {
                domain: domain.clone(),
                name: name.clone(),
            };
            storage.store_entity(&entity)?;
            trace!("Stored entity: {:?}", &entity);
            domain_entity_map.insert(domain.to_string(), entity);
        }

        let jack_entity = &domain_entity_map[&Domain::Jack.to_string()];
        let jill_entity = &domain_entity_map[&Domain::Jill.to_string()];

        let p_jack_lonely = cointoss();
        let p_jill_exciting: f64 = cointoss();
        let p_jill_likes_jack: f64 = cointoss();
        let p_jack_likes_jill = weighted_cointoss(0.8 * numeric_or(p_jack_lonely, p_jill_exciting));
        let p_jack_dates_jill = numeric_and(p_jack_likes_jill, p_jill_likes_jack);

        {
            trace!("Jack entity part 2: {:?}", jack_entity);
            let jack = constant(jack_entity.domain, jack_entity.name.clone());
            let jack_lonely = proposition(vec![subject(jack), relation(lonely.clone())]);

            trace!(
                "Jack Lonely: {:?}, Probability: {}",
                jack_lonely.search_string(),
                p_jack_lonely
            );
            storage.store_proposition(&jack_lonely, p_jack_lonely)?;
        }

        {
            let jill = constant(jill_entity.domain, jill_entity.name.clone());
            let jill_exciting = proposition(vec![subject(jill), relation(exciting.clone())]);

            trace!(
                "Jill Exciting: {:?}, Probability: {}",
                jill_exciting.search_string(),
                p_jill_exciting
            ); 
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
                jill_likes_jack.search_string(),
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
                jack_likes_jill.search_string(),
                p_jack_likes_jill
            ); // Logging
            storage.store_proposition(&jack_likes_jill, p_jack_likes_jill)?;
            storage.maybe_add_to_training(is_training, &jack_likes_jill)?;
        }
        {
            let jill = constant(jill_entity.domain, jill_entity.name.clone());
            let jack = constant(jack_entity.domain, jack_entity.name.clone());

            // "dates(jack, jill)" based on "likes(jack, jill) and likes(jill, jack)"
            let jack_dates_jill =
                proposition(vec![subject(jack), relation(date.clone()), object(jill)]);
            trace!(
                "Jack dates Jill: {:?}, Probability: {}",
                jack_dates_jill.search_string(),
                p_jack_dates_jill
            ); // Logging

            let adusted_p = p_jack_dates_jill * 0.7;
            let effective_p = weighted_cointoss(adusted_p);
            storage.store_proposition(&jack_dates_jill, effective_p)?;
            storage.maybe_add_to_training(is_training, &jack_dates_jill)?;
            storage.maybe_add_to_test(is_test, &jack_dates_jill)?;
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
            conjunction(vec![
                proposition(vec![
                    subject(xjill.clone()),
                    relation(like.clone()),
                    object(xjack.clone()),
                ]),
                proposition(vec![
                    subject(xjack.clone()),
                    relation(like.clone()),
                    object(xjill.clone()),
                ]),
            ]),
            proposition(vec![
                subject(xjack.clone()),
                relation(date.clone()),
                object(xjill.clone()),
            ]),
            vec![
                RoleMap::new(HashMap::from([
                    ("subject".to_string(), "object".to_string()),
                    ("object".to_string(), "subject".to_string()),
                ])),
                RoleMap::new(HashMap::from([
                    ("subject".to_string(), "subject".to_string()),
                    ("object".to_string(), "object".to_string()),
                ])),
            ],
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
