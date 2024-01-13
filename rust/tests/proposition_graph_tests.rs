use std::collections::HashMap;

use bayes_star::model::config::ConfigurationOptions;
use bayes_star::{
    common::{
        graph::InferenceGraph, interface::ScenarioMaker,
        resources::FactoryResources,
    },
    model::{
        creators::{
            conjunction, constant, implication, object, predicate, relation, subject, variable,
        },
        objects::{Domain, RoleMap},
    },
    scenarios::dating_simple::SimpleDating,
};
use log::trace;

#[test]
fn test_get_proposition_forward_links() {
    let config = ConfigurationOptions {
        entities_per_domain: 12,
        print_training_loss: false,
    };
    let mut resources = FactoryResources::new(&config).expect("Couldn't create resources.");
    resources.redis.drop_all_dbs().unwrap();
    let scenario_maker = SimpleDating {};
    let result = scenario_maker.setup_scenario(&resources);
    trace!("scenario result: {:?}", result);
    let predicate_graph = InferenceGraph::new_mutable(&resources).unwrap();

    let exciting = constant(Domain::Verb, "exciting".to_string());
    let lonely = constant(Domain::Verb, "lonely".to_string());
    let like = constant(Domain::Verb, "like".to_string());
    let date = constant(Domain::Verb, "date".to_string());
    let xjack = variable(Domain::Jack);
    let xjill = variable(Domain::Jill);

    let predicate = predicate(vec![
        subject(xjack.clone()),
        relation(like.clone()),
        object(xjill.clone()),
    ]);

    let result = predicate_graph.predicate_forward_links(&predicate).unwrap();
    println!("{:?}", &result);
}
