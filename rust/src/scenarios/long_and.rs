use crate::common::graph::InferenceGraph;
use crate::common::interface::BeliefTable;
use crate::common::model::InferenceModel;
use crate::common::proposition_db::RedisBeliefTable;
use crate::common::redis::RedisManager;
use crate::common::resources::{self, NamespaceBundle};
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

const LINK_HEIGHT: u32 = 10;

impl ScenarioMaker for Scenario {
    fn setup_scenario(&self, resources: &NamespaceBundle) -> Result<(), Box<dyn Error>> {
        let mut graph = InferenceGraph::new_mutable(resources)?;
        let proposition_db = RedisBeliefTable::new_mutable(&resources)?;
        let mut plan = TrainingPlan::new(&resources)?;
        let config = &resources.config;
        let total_members_each_class = config.entities_per_domain;
        let domain = Domain::MAN.to_string();
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
            let p_jack_beta = weighted_cointoss(0.3f64);
            let p_jack_gamma = p_jack_alpha && p_jack_beta;
            let jack = constant(jack_entity.domain, jack_entity.name.clone());
            for level in 0..LINK_HEIGHT {
                let function = format!("alpha{}", level);
                let jack_alpha = proposition(function, vec![sub(jack.clone())]);
                if level == 0 {
                    graph.ensure_existence_backlinks_for_proposition(&jack_alpha)?;
                }
                proposition_db.store_proposition_boolean(&jack_alpha, p_jack_alpha)?;
                plan.maybe_add_to_training(is_training, &jack_alpha)?;
            }
            for level in 0..LINK_HEIGHT {
                let function = format!("beta{}", level);
                let jack_beta = proposition(function, vec![sub(jack.clone())]);
                if level == 0 {
                    graph.ensure_existence_backlinks_for_proposition(&jack_beta)?;
                }
                proposition_db.store_proposition_boolean(&jack_beta, p_jack_beta)?;
                plan.maybe_add_to_training(is_training, &jack_beta)?;
            }
            {
                let function = format!("gamma");
                let jack_gamma = proposition(function, vec![sub(jack.clone())]);
                proposition_db.store_proposition_boolean(&jack_gamma, p_jack_gamma)?;
                plan.maybe_add_to_training(is_training, &jack_gamma)?;
                plan.maybe_add_to_test(is_test, &jack_gamma)?;
            }
        }
        let xjack = variable(Domain::MAN.to_string());
        let mut implications = vec![];
        let channel_names = ["alpha", "beta"];
        for channel_name in channel_names {
            for level in 0..(LINK_HEIGHT - 1) {
                let fn1 = format!("{}{}", channel_name, level);
                let fn2 = format!("{}{}", channel_name, level + 1);
                implications.push(implication(
                    conjunction(vec![predicate(fn1, vec![sub(xjack.clone())])]),
                    predicate(fn2, vec![sub(xjack.clone())]),
                    vec![RoleMap::new(HashMap::from([(
                        "sub".to_string(),
                        "sub".to_string(),
                    )]))],
                ));
            }
        }
        implications.push(implication(
            conjunction(vec![
                predicate(format!("{}{}", "alpha", LINK_HEIGHT - 1), vec![sub(xjack.clone())]),
                predicate(format!("{}{}", "beta", LINK_HEIGHT - 1), vec![sub(xjack.clone())]),
            ]),
            predicate(format!("gamma"), vec![sub(xjack.clone())]),
            vec![
                RoleMap::new(HashMap::from([(
                "sub".to_string(),
                "sub".to_string(),
            )])),
                RoleMap::new(HashMap::from([(
                "sub".to_string(),
                "sub".to_string(),
            )]))
            ],
        ));
        graph.store_predicate_implications(&implications)?;
        Ok(())
    }
}
