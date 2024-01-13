use std::collections::HashMap;

use bayes_star::model::config::ConfigurationOptions;
use bayes_star::{
    common::{
        graph::PredicateGraph, interface::ScenarioMaker, redis::RedisManager,
        resources::FactoryResources,
    },
    inference::implications,
    model::{
        creators::{
            conjunction, constant, implication, object, predicate, relation, subject, variable,
        },
        objects::{Domain, RoleMap},
    },
    scenarios::dating_simple::SimpleDating,
};
use log::info;

#[test]
fn test_store_entity() {
    let config = ConfigurationOptions {
        entities_per_domain: 12,
        print_training_loss: false,
    };
    let mut resources = FactoryResources::new(&config).expect("Couldn't create resources.");
    resources.redis.drop_all_dbs().unwrap();
    let scenario_maker = SimpleDating {};
    let result = scenario_maker.setup_scenario(&resources);
    println!("scenario result: {:?}", result);
    let predicate_graph = PredicateGraph::new(&resources.redis).unwrap();
    let implications = predicate_graph.get_all_implications().unwrap();
    for implication in &implications {
        println!("implication {:?}", implication);
    }

    let exciting = constant(Domain::Verb, "exciting".to_string());
    let lonely = constant(Domain::Verb, "lonely".to_string());
    let like = constant(Domain::Verb, "like".to_string());
    let date = constant(Domain::Verb, "date".to_string());
    let xjack = variable(Domain::Jack);
    let xjill = variable(Domain::Jill);
    let implications = vec![
        // if jack is lonely, he will date any jill
        implication(
            conjunction(vec![predicate(vec![
                subject(xjack.clone()),
                relation(lonely.clone()),
            ])]),
            predicate(vec![
                subject(xjack.clone()),
                relation(like.clone()),
                object(xjill.clone()),
            ]),
            vec![RoleMap::new(HashMap::from([(
                "subject".to_string(),
                "subject".to_string(),
            )]))],
        ),
        // if jill is exciting, any jack will date her
        implication(
            conjunction(vec![predicate(vec![
                subject(xjill.clone()),
                relation(exciting.clone()),
            ])]),
            predicate(vec![
                subject(xjack.clone()),
                relation(like.clone()),
                object(xjill.clone()),
            ]),
            vec![RoleMap::new(HashMap::from([(
                "object".to_string(),
                "subject".to_string(),
            )]))],
        ),
        // if jill likes jack, then jack dates jill
        implication(
            conjunction(vec![
                predicate(vec![
                    subject(xjill.clone()),
                    relation(like.clone()),
                    object(xjack.clone()),
                ]),
                predicate(vec![
                    subject(xjack.clone()),
                    relation(like.clone()),
                    object(xjill.clone()),
                ]),
            ]),
            predicate(vec![
                subject(xjack.clone()),
                relation(date.clone()),
                object(xjill.clone()),
            ]),
            vec![
                RoleMap::new(HashMap::from([
                    ("subject".to_string(), "object".to_string()),
                    ("object".to_string(), "subject".to_string()),
                ])),
                RoleMap::new(HashMap::from([
                    ("subject".to_string(), "subject".to_string()),
                    ("object".to_string(), "object".to_string()),
                ])),
            ],
        ),
    ];
}
