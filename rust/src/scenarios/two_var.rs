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

use super::helpers::weighted_cointoss;

pub struct TwoVariable {}

impl ScenarioMaker for TwoVariable {
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
        let jacks: Vec<Entity> = graph.get_entities_in_domain(&jack_domain)?;
        let mut propositions = vec![];
        for i in 0..total_members_each_class {
            let is_test = i == 0;
            let is_training = !is_test;
            let mut domain_entity_map: HashMap<String, Entity> = HashMap::new();
            for domain in [Domain::Jack].iter() {
                let prefix = if is_test { "test" } else { "train" };
                let name = format!("{}_{:?}{}", &prefix, domain, i);
                let entity = Entity {
                    domain: domain.clone(),
                    name: name.clone(),
                };
                graph.store_entity(&entity)?;
                domain_entity_map.insert(domain.to_string(), entity);
            }
            let jack_entity = &domain_entity_map[&Domain::Jack.to_string()];
            let p_jack_exciting = weighted_cointoss(0.3f64);
            {
                let jack = constant(jack_entity.domain, jack_entity.name.clone());
                let jack_exciting = proposition("exciting".to_string(), vec![sub(jack)]);
                graph.ensure_existence_backlinks_for_proposition(&jack_exciting)?;
                proposition_db.store_proposition_boolean(&jack_exciting, p_jack_exciting)?;
                plan.maybe_add_to_training(is_training, &jack_exciting)?;
                propositions.push(jack_exciting.clone());
            }
            {
                let jack = constant(jack_entity.domain, jack_entity.name.clone());
                let jack_rich = proposition("rich".to_string(), vec![sub(jack)]);
                graph.ensure_existence_backlinks_for_proposition(&jack_rich)?;
                proposition_db.store_proposition_boolean(&jack_rich, p_jack_exciting)?;
                plan.maybe_add_to_training(is_training, &jack_rich)?;
                propositions.push(jack_rich.clone());
                plan.maybe_add_to_test(is_test, &jack_rich)?;
            }
        }
        let xjack = variable(Domain::Jack);
        let implications = vec![
            implication(
                conjunction(vec![predicate("exciting".to_string(), vec![
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
        ];
        for implication in implications.iter() {
            trace!("Storing implication: {:?}", implication);
            graph.store_predicate_implication(implication)?;
        }
        Ok(())
    }
}
