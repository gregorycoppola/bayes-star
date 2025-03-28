use crate::common::proposition_db::RedisBeliefTable;
use crate::common::graph::InferenceGraph;
use crate::common::interface::BeliefTable;
use crate::common::model::InferenceModel;
use crate::common::redis::RedisManager;
use crate::common::resources::{self, NamespaceBundle};
use crate::common::train::TrainingPlan;
use crate::model::creators::predicate;
use crate::scenarios::helpers::weighted_cointoss;
use crate::{
    common::interface::ScenarioMaker,
    model::{
        creators::{conjunction, constant, implication, obj, proposition, sub, variable},
        objects::{Domain, Entity, RoleMap},
    },
};
use std::{collections::HashMap, error::Error};

pub struct EligibilityTriangle {}

impl ScenarioMaker for EligibilityTriangle {
    fn setup_scenario(
        &self,
        resources: &NamespaceBundle,
    ) -> Result<(), Box<dyn Error>> {
        let mut graph = InferenceGraph::new_mutable(resources)?;
        let proposition_db = RedisBeliefTable::new_mutable(&resources)?;
        let mut plan = TrainingPlan::new(&resources)?;
        let config = &resources.config;
        let total_members_each_class = config.entities_per_domain;
        let jack_domain = Domain::MAN.to_string();
        for i in 0..total_members_each_class {
            let is_test = i == 0;
            let is_training = !is_test;
            let prefix = if is_test { "test" } else { "train" };
            let name = format!("{}_{:?}{}", &prefix, Domain::MAN, i);
            let domain = Domain::MAN.to_string();
            let jack_entity = Entity {
                domain,
                name: name.clone(),
            };
            graph.store_entity(&jack_entity)?;
            let jack = constant(jack_entity.domain, jack_entity.name.clone());
            let p_jack_charming = weighted_cointoss(0.3f64);
            let jack_charming = proposition("charming".to_string(), vec![sub(jack.clone())]);
            proposition_db.store_proposition_boolean(&jack_charming, p_jack_charming)?;
            plan.maybe_add_to_training(is_training, &jack_charming)?;
            graph.ensure_existence_backlinks_for_proposition(&jack_charming)?;
            let p_jack_rich: bool = if p_jack_charming {
                weighted_cointoss(0.7f64)
            } else {
                weighted_cointoss(0.2f64)
            };
            let jack_rich = proposition("rich".to_string(), vec![sub(jack.clone())]);
            proposition_db.store_proposition_boolean(&jack_rich, p_jack_rich)?;
            plan.maybe_add_to_training(is_training, &jack_rich)?;
            let p_jack_baller = p_jack_charming && p_jack_rich;
            let jack_baller = proposition("baller".to_string(), vec![sub(jack.clone())]);
            proposition_db.store_proposition_boolean(&jack_baller, p_jack_baller)?;
            plan.maybe_add_to_training(is_training, &jack_baller)?;
            plan.maybe_add_to_test(is_test, &jack_baller)?;
        }

        let xjack = variable(Domain::MAN.to_string());
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
        graph.store_predicate_implications(&implications)?;
        Ok(())
    }
}
