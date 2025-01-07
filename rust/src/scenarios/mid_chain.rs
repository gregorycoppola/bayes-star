use crate::common::graph::InferenceGraph;
use crate::common::interface::BeliefTable;
use crate::common::model::InferenceModel;
use crate::common::proposition_db::RedisBeliefTable;
use crate::common::redis::RedisManager;
use crate::common::resources::{self, FactoryResources};
use crate::common::train::TrainingPlan;
use crate::model::choose::extract_existence_factor_for_proposition;
use crate::model::creators::predicate;
use crate::{
    common::interface::ScenarioMaker,
    model::{
        creators::{conjunction, constant, implication, obj, proposition, sub, variable},
        objects::{Domain, Entity, RoleMap},
    },
};
use crate::{print_red, print_yellow};
use rand::Rng; // Import Rng trait
use std::{collections::HashMap, error::Error};

use super::helpers::weighted_cointoss;

pub struct Scenario {}

const LINK_HEIGHT: u32 = 5;

impl ScenarioMaker for Scenario {
    fn setup_scenario(&self, resources: &FactoryResources) -> Result<(), Box<dyn Error>> {
        let mut graph = InferenceGraph::new_mutable(resources)?;
        let proposition_db = RedisBeliefTable::new_mutable(&resources)?;
        let mut plan = TrainingPlan::new(&resources)?;
        let config = &resources.config;
        let total_members_each_class = config.entities_per_domain;
        let domain = Domain::Man.to_string();
        for i in 0..total_members_each_class {
            let is_test = i == 0;
            let is_training = !is_test;
            let prefix = if is_test { "test" } else { "train" };
            let name = format!("{}_{:?}{}", &prefix, domain, i);
            let jack_entity = Entity {
                domain: domain.clone(),
                name: name.clone(),
            };
            graph.store_entity(&jack_entity)?;

            let p_jack_alpha = weighted_cointoss(0.3f64);
            for level in 0..LINK_HEIGHT {
                let jack = constant(jack_entity.domain, jack_entity.name.clone());
                let function = format!("alpha{}", level);
                let jack_alpha = proposition(function, vec![sub(jack)]);
                if level == 0 {
                    graph.ensure_existence_backlinks_for_proposition(&jack_alpha)?;
                }
                proposition_db.store_proposition_boolean(&jack_alpha, p_jack_alpha)?;
                plan.maybe_add_to_training(is_training, &jack_alpha)?;

                if level == LINK_HEIGHT - 1 {
                    plan.maybe_add_to_test(is_test, &jack_alpha)?;
                }
            }
        }
        let xjack = variable(Domain::Man.to_string());
        let mut implications = vec![];
        for level in 0..(LINK_HEIGHT-1) {
            let fn1 = format!("alpha{}", level);
            let fn2 = format!("alpha{}", level + 1);
            implications.push(implication(
                conjunction(vec![predicate(
                    fn1,
                    vec![sub(xjack.clone())],
                )]),
                predicate(fn2, vec![sub(xjack.clone())]),
                vec![RoleMap::new(HashMap::from([(
                    "sub".to_string(),
                    "sub".to_string(),
                )]))],
            ));
        }
        graph.store_predicate_implications(&implications)?;
        Ok(())
    }
}
