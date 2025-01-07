use crate::common::proposition_db::RedisBeliefTable;
use crate::common::graph::InferenceGraph;
use crate::common::interface::BeliefTable;
use crate::common::model::InferenceModel;
use crate::common::redis::RedisManager;
use crate::common::resources::{self, FactoryResources};
use crate::common::train::TrainingPlan;
use crate::model::creators::predicate;
use crate::{
    common::interface::ScenarioMaker,
    model::{
        creators::{conjunction, constant, implication, obj, proposition, sub, variable},
        objects::{Domain, Entity, RoleMap},
    },
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

pub struct SimpleDating {}

impl ScenarioMaker for SimpleDating {
    fn setup_scenario(
        &self,
        resources: &FactoryResources,
    ) -> Result<(), Box<dyn Error>> {
        let mut graph = InferenceGraph::new_mutable(resources)?;
        let proposition_db = RedisBeliefTable::new_mutable(&resources)?;
        let mut plan = TrainingPlan::new(&resources)?;
        let config = &resources.config;
        let total_members_each_class = config.entities_per_domain;
        let entity_domains = [Domain::Man.to_string(), Domain::Woman.to_string()];

        // Retrieve entities in the Man domain
        let jack_domain = Domain::Man.to_string(); // Convert enum to string and make lowercase
        let jacks: Vec<Entity> = graph.get_entities_in_domain(&jack_domain)?;
        trace!("Initial number of jacks: {}", jacks.len());
        // Retrieve entities in the Woman domain
        let jill_domain = Domain::Woman.to_string(); // Convert enum to string and make lowercase
        let jills = graph.get_entities_in_domain(&jill_domain)?;
        trace!("Initial number of jills: {}", jills.len());

        for i in 0..total_members_each_class {
            let is_test = i  == 0;
            let is_training = !is_test;
            let mut domain_entity_map: HashMap<String, Entity> = HashMap::new();
            for domain in entity_domains.iter() {
                let prefix = if is_test { "test" } else { "train" };
                let name = format!("{}_{:?}{}", &prefix, domain, i); // Using Debug formatting for Domain enum
                let entity = Entity {
                    domain: domain.clone(),
                    name: name.clone(),
                };
                graph.store_entity(&entity)?;
                trace!("Stored entity: {:?}", &entity);
                domain_entity_map.insert(domain.to_string(), entity);
            }

            let jack_entity = &domain_entity_map[&Domain::Man.to_string()];
            let jill_entity = &domain_entity_map[&Domain::Woman.to_string()];

            let p_jack_lonely = weighted_cointoss(0.3f64);
            let p_jill_exciting: f64 = weighted_cointoss(0.6f64);
            let p_jill_likes_jack: f64 = weighted_cointoss(0.4f64);
            let p_jack_likes_jill =
                weighted_cointoss(numeric_or(p_jack_lonely, p_jill_exciting));
            let p_jack_dates_jill = numeric_and(p_jack_likes_jill, p_jill_likes_jack);

            {
                trace!("Man entity part 2: {:?}", jack_entity);
                let jack = constant(jack_entity.domain, jack_entity.name.clone());
                let jack_lonely = proposition("lonely".to_string(), vec![sub(jack)]);

                trace!(
                    "Man Lonely: {:?}, Probability: {}",
                    jack_lonely.predicate.hash_string(),
                    p_jack_lonely
                );
                proposition_db.store_proposition_probability(&jack_lonely, p_jack_lonely)?;
                plan.maybe_add_to_training(is_training, &jack_lonely)?;
                graph.ensure_existence_backlinks_for_proposition(&jack_lonely)?;
            }

            {
                let jill = constant(jill_entity.domain, jill_entity.name.clone());
                let jill_exciting = proposition("exciting".to_string(),
                vec![sub(jill)]);

                trace!(
                    "Woman Exciting: {:?}, Probability: {}",
                    jill_exciting.predicate.hash_string(),
                    p_jill_exciting
                );
                proposition_db.store_proposition_probability(&jill_exciting, p_jill_exciting)?;
                plan.maybe_add_to_training(is_training, &jill_exciting)?;
                graph.ensure_existence_backlinks_for_proposition(&jill_exciting)?;
            }

            {
                let jill = constant(jill_entity.domain, jill_entity.name.clone());
                let jack = constant(jack_entity.domain, jack_entity.name.clone());

                // "likes(jill, jack)"
                let jill_likes_jack = proposition(
                    "like".to_string(),
                    vec![
                    sub(jill.clone()),
                    obj(jack.clone()),
                ]);
                trace!(
                    "Woman likes Man: {:?}, Probability: {}",
                    jill_likes_jack.predicate.hash_string(),
                    p_jill_likes_jack
                ); // Logging
                proposition_db.store_proposition_probability(&jill_likes_jack, p_jill_likes_jack)?;
                plan.maybe_add_to_training(is_training, &jill_likes_jack)?;
                graph.ensure_existence_backlinks_for_proposition(&jill_likes_jack)?;
            }

            {
                let jill = constant(jill_entity.domain, jill_entity.name.clone());
                let jack = constant(jack_entity.domain, jack_entity.name.clone());
                let jack_likes_jill = proposition(
                    "like".to_string(),
                    vec![
                    sub(jack.clone()),
                    obj(jill.clone()),
                ]);
                trace!(
                    "Man likes Woman: {:?}, Probability: {}",
                    jack_likes_jill.predicate.hash_string(),
                    p_jack_likes_jill
                ); // Logging
                if is_training {
                    proposition_db.store_proposition_probability(&jack_likes_jill, p_jack_likes_jill)?;
                }
                plan.maybe_add_to_training(is_training, &jack_likes_jill)?;
                // graph.ensure_existence_backlinks_for_proposition(&jack_likes_jill)?;
            }
            {
                let jill = constant(jill_entity.domain, jill_entity.name.clone());
                let jack = constant(jack_entity.domain, jack_entity.name.clone());

                // "dates(jack, jill)" based on "likes(jack, jill) and likes(jill, jack)"
                let jack_dates_jill =
                    proposition(
                        "date".to_string(),
                        vec![sub(jack),  obj(jill)]);
                trace!(
                    "Man dates Woman: {:?}, Probability: {}",
                    jack_dates_jill.predicate.hash_string(),
                    p_jack_dates_jill
                ); // Logging

                if is_training {
                    proposition_db.store_proposition_probability(&jack_dates_jill, p_jack_dates_jill)?;
                }
                plan.maybe_add_to_training(is_training, &jack_dates_jill)?;
                plan.maybe_add_to_test(is_test, &jack_dates_jill)?;
                // graph.ensure_existence_backlinks_for_proposition(&jack_dates_jill)?;
            }
        }

        let xjack = variable(Domain::Man.to_string());
        let xjill = variable(Domain::Woman.to_string());

        let implications = vec![
            // if jack is lonely, he will date any jill
            implication(
                conjunction(vec![predicate("lonely".to_string(), vec![
                    sub(xjack.clone()),
                ])]),
                predicate("like".to_string(), 
                vec![
                    sub(xjack.clone()),
                    obj(xjill.clone()),
                ]),
                vec![RoleMap::new(HashMap::from([(
                    "sub".to_string(),
                    "sub".to_string(),
                )]))],
            ),
            // if jill is exciting, any jack will date her
            implication(
                conjunction(vec![predicate("exciting".to_string(),
                vec![
                    sub(xjill.clone()),
                ])]),
                predicate("like".to_string(),
                vec![
                    sub(xjack.clone()),
                    obj(xjill.clone()),
                ]),
                vec![RoleMap::new(HashMap::from([(
                    "obj".to_string(),
                    "sub".to_string(),
                )]))],
            ),
            // if jill likes jack, then jack dates jill
            implication(
                conjunction(vec![
                    predicate("like".to_string(),
                    vec![
                        sub(xjill.clone()),
                        obj(xjack.clone()),
                    ]),
                    predicate("like".to_string(), vec![
                        sub(xjack.clone()),
                        obj(xjill.clone()),
                    ]),
                ]),
                predicate("date".to_string(),
                vec![
                    sub(xjack.clone()),
                    obj(xjill.clone()),
                ]),
                vec![
                    RoleMap::new(HashMap::from([
                        ("sub".to_string(), "obj".to_string()),
                        ("obj".to_string(), "sub".to_string()),
                    ])),
                    RoleMap::new(HashMap::from([
                        ("sub".to_string(), "sub".to_string()),
                        ("obj".to_string(), "obj".to_string()),
                    ])),
                ],
            ),
        ];

        for implication in implications.iter() {
            trace!("Storing implication: {:?}", implication);
            graph.store_predicate_implication(implication)?;
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
}
