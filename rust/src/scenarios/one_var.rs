use crate::common::proposition_db::RedisBeliefTable;
use crate::common::graph::InferenceGraph;
use crate::common::interface::BeliefTable;
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

pub struct OneVariable {}

impl ScenarioMaker for OneVariable {
    fn setup_scenario(
        &self,
        resources: &FactoryResources,
    ) -> Result<(), Box<dyn Error>> {
        let mut graph = InferenceGraph::new_mutable(resources)?;
        let proposition_db = RedisBeliefTable::new_mutable(&resources.redis)?;
        let mut plan = TrainingPlan::new(&resources.redis)?;
        let config = &resources.config;
        let total_members_each_class = config.entities_per_domain;
        let jack_domain = Domain::Jack;
        for i in 0..total_members_each_class {
            let is_test = i % 10 == 9;
            let is_training = !is_test;
            let domain = Domain::Jack;
            let prefix = if is_test { "test" } else { "train" };
            let name = format!("{}_{:?}{}", &prefix, domain, i);
            let jack_entity = Entity {
                domain: domain.clone(),
                name: name.clone(),
            };
            graph.store_entity(&jack_entity)?;
            let p_jack_exciting = weighted_cointoss(0.3f64);
            {
                let jack = constant(jack_entity.domain, jack_entity.name.clone());
                let jack_exciting = proposition("exciting".to_string(), vec![sub(jack)]);
                graph.ensure_existence_backlinks_for_proposition(&jack_exciting)?;
                proposition_db.store_proposition_probability(&jack_exciting, p_jack_exciting)?;
                plan.maybe_add_to_training(is_training, &jack_exciting)?;
                plan.maybe_add_to_test(is_test, &jack_exciting)?;
            }
        }
        Ok(())
    }
}
