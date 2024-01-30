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

pub struct EligibilityTriangle {}

impl ScenarioMaker for EligibilityTriangle {
    fn setup_scenario(
        &self,
        resources: &FactoryResources,
    ) -> Result<(), Box<dyn Error>> {
        let mut graph = InferenceGraph::new_mutable(resources)?;
        let proposition_db = RedisBeliefTable::new_mutable(&resources.redis)?;
        let mut plan = TrainingPlan::new(&resources.redis)?;
        let config = &resources.config;
        let total_members_each_class = config.entities_per_domain;
        let entity_domains = [Domain::Jack, Domain::Jill];

        // Retrieve entities in the Jack domain
        let jack_domain = Domain::Jack.to_string(); // Convert enum to string and make lowercase
        let jacks: Vec<Entity> = graph.get_entities_in_domain(&jack_domain)?;
        trace!("Initial number of jacks: {}", jacks.len());
        // Retrieve entities in the Jill domain
        let jill_domain = Domain::Jill.to_string(); // Convert enum to string and make lowercase
        let jills = graph.get_entities_in_domain(&jill_domain)?;
        trace!("Initial number of jills: {}", jills.len());

        for i in 0..total_members_each_class {
            let is_test = i % 10 == 9;
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

            let jack_entity = &domain_entity_map[&Domain::Jack.to_string()];
            let jill_entity = &domain_entity_map[&Domain::Jill.to_string()];

            let p_jack_charming = weighted_cointoss(0.3f64);
            let p_jill_exciting: f64 = weighted_cointoss(0.6f64);
            let p_jill_likes_jack: f64 = weighted_cointoss(0.4f64);
            let p_jack_likes_jill =
                weighted_cointoss(numeric_or(p_jack_charming, p_jill_exciting));
            let p_jack_dates_jill = numeric_and(p_jack_likes_jill, p_jill_likes_jack);

            {
                trace!("Jack entity part 2: {:?}", jack_entity);
                let jack = constant(jack_entity.domain, jack_entity.name.clone());
                let jack_charming = proposition("charming".to_string(), vec![sub(jack)]);

                trace!(
                    "Jack charming: {:?}, Probability: {}",
                    jack_charming.predicate.hash_string(),
                    p_jack_charming
                );
                proposition_db.store_proposition_probability(&jack_charming, p_jack_charming)?;
                plan.maybe_add_to_training(is_training, &jack_charming)?;
                graph.ensure_existence_backlinks_for_proposition(&jack_charming)?;
            }

            {
                let jill = constant(jill_entity.domain, jill_entity.name.clone());
                let jill_exciting = proposition("exciting".to_string(),
                vec![sub(jill)]);

                trace!(
                    "Jill Exciting: {:?}, Probability: {}",
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
                    "Jill likes Jack: {:?}, Probability: {}",
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
                    "Jack likes Jill: {:?}, Probability: {}",
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
                    "Jack dates Jill: {:?}, Probability: {}",
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

        let xjack = variable(Domain::Jack);
        let xjill = variable(Domain::Jill);

        let implications = vec![
            // if jack is charming, he will date any jill
            implication(
                conjunction(vec![predicate("charming".to_string(), vec![
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
