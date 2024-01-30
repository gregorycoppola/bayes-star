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

            let jack = constant(jack_entity.domain, jack_entity.name.clone());
            let jack_charming = proposition("charming".to_string(), vec![sub(jack)]);
            proposition_db.store_proposition_probability(&jack_charming, p_jack_charming)?;
            plan.maybe_add_to_training(is_training, &jack_charming)?;
            graph.ensure_existence_backlinks_for_proposition(&jack_charming)?;

        }

        let xjack = variable(Domain::Jack);
        let xjill = variable(Domain::Jill);

        let implications = vec![
            implication(
                conjunction(vec![predicate("charming".to_string(), vec![
                    sub(xjack.clone()),
                ])]),
                predicate("rich".to_string(), 
                vec![
                    sub(xjack.clone()),
                ]),
                vec![RoleMap::new(HashMap::from([(
                    "sub".to_string(),
                    "sub".to_string(),
                )]))],
            ),
            implication(
                conjunction(vec![
                    predicate("rich".to_string(),
                    vec![
                        sub(xjack.clone()),
                    ]),
                    predicate("charming".to_string(), vec![
                        sub(xjack.clone()),
                    ]),
                ]),
                predicate("baller".to_string(),
                vec![
                    sub(xjack.clone()),
                ]),
                vec![
                    RoleMap::new(HashMap::from([
                        ("sub".to_string(), "sub".to_string()),
                    ])),
                    RoleMap::new(HashMap::from([
                        ("sub".to_string(), "sub".to_string()),
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
