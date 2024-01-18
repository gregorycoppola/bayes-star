use crate::common::proposition_db::RedisFactDB;
use crate::common::graph::InferenceGraph;
use crate::common::interface::PropositionDB;
use crate::common::model::InferenceModel;
use crate::common::redis::RedisManager;
use crate::common::resources::{self, FactoryResources};
use crate::common::train::TrainingPlan;
use crate::model::choose::extract_existence_factor_for_proposition;
use crate::model::creators::predicate;
use crate::{print_red, print_yellow};
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

pub struct TwoVariable {}

impl ScenarioMaker for TwoVariable {
    fn setup_scenario(
        &self,
        resources: &FactoryResources,
    ) -> Result<(), Box<dyn Error>> {
        let mut graph = InferenceGraph::new_mutable(resources)?;
        let mut proposition_db = RedisFactDB::new_mutable(&resources.redis)?;
        let mut plan = TrainingPlan::new(&resources.redis)?;
        let config = &resources.config;
        let total_members_each_class = config.entities_per_domain;

        // Retrieve entities in the Jack domain
        let jack_domain = Domain::Jack.to_string(); // Convert enum to string and make lowercase
        let jacks: Vec<Entity> = graph.get_entities_in_domain(&jack_domain)?;
        trace!("Initial number of jacks: {}", jacks.len());

        let mut propositions = vec![];
        for i in 0..total_members_each_class {
            let is_test = i % 10 == 9;
            let is_training = !is_test;
            let mut domain_entity_map: HashMap<String, Entity> = HashMap::new();

            for domain in [Domain::Jack].iter() {
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
            let p_jack_exciting = weighted_cointoss(0.3f64);
            {
                trace!("Jack entity part 2: {:?}", jack_entity);
                let jack = constant(jack_entity.domain, jack_entity.name.clone());
                let jack_exciting = proposition("exciting".to_string(), vec![sub(jack)]);

                print_yellow!(
                    "Jack exciting: {:?}, Probability: {}",
                    jack_exciting.predicate.hash_string(),
                    p_jack_exciting
                );
                graph.ensure_existence_backlinks_for_proposition(&jack_exciting)?;
                proposition_db.store_proposition_probability(&jack_exciting, p_jack_exciting)?;
                plan.maybe_add_to_training(is_training, &jack_exciting)?;
                propositions.push(jack_exciting.clone());
            }
            {
                trace!("Jack entity part 2: {:?}", jack_entity);
                let jack = constant(jack_entity.domain, jack_entity.name.clone());
                let jack_rich = proposition("rich".to_string(), vec![sub(jack)]);

                print_yellow!(
                    "Jack rich: {:?}, Probability: {}",
                    jack_rich.predicate.hash_string(),
                    p_jack_exciting
                );
                graph.ensure_existence_backlinks_for_proposition(&jack_rich)?;
                proposition_db.store_proposition_probability(&jack_rich, p_jack_exciting)?;
                plan.maybe_add_to_training(is_training, &jack_rich)?;
                propositions.push(jack_rich.clone());
                plan.maybe_add_to_test(is_test, &jack_rich)?;
            }
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
