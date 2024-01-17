use std::collections::HashMap;

use bayes_star::model::config::ConfigurationOptions;
use bayes_star::{
    common::{
        graph::InferenceGraph, interface::ScenarioMaker,
        resources::FactoryResources,
    },
    model::{
        creators::{
            conjunction, constant, implication, obj, predicate, sub, variable,
        },
        objects::{Domain, RoleMap},
    },
    scenarios::dating_simple::SimpleDating,
};
use log::trace;

#[test]
fn test_get_all_implications() {
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
    let computed = predicate_graph.get_all_implications().unwrap();

    let exciting = constant(Domain::Verb, "exciting".to_string());
    let lonely = constant(Domain::Verb, "lonely".to_string());
    let like = constant(Domain::Verb, "like".to_string());
    let date = constant(Domain::Verb, "date".to_string());
    let xjack = variable(Domain::Jack);
    let xjill = variable(Domain::Jill);
    let expected = vec![
        // if jack is lonely, he will date any jill
        implication(
            conjunction(vec![predicate(vec![
                sub(xjack.clone()),
                relation(lonely.clone()),
            ])]),
            predicate(vec![
                sub(xjack.clone()),
                relation(like.clone()),
                obj(xjill.clone()),
            ]),
            vec![RoleMap::new(HashMap::from([(
                "sub".to_string(),
                "sub".to_string(),
            )]))],
        ),
        // if jill is exciting, any jack will date her
        implication(
            conjunction(vec![predicate(vec![
                sub(xjill.clone()),
                relation(exciting.clone()),
            ])]),
            predicate(vec![
                sub(xjack.clone()),
                relation(like.clone()),
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
                predicate(vec![
                    sub(xjill.clone()),
                    relation(like.clone()),
                    obj(xjack.clone()),
                ]),
                predicate(vec![
                    sub(xjack.clone()),
                    relation(like.clone()),
                    obj(xjill.clone()),
                ]),
            ]),
            predicate(vec![
                sub(xjack.clone()),
                relation(date.clone()),
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
    // TODO: Actually check the implications.
    assert_eq!(computed.len(), expected.len());
}

// #[test]
// fn test_get_predicate_forward_links() {
//     let config = ConfigurationOptions {
//         entities_per_domain: 12,
//         print_training_loss: false,
//     };
//     let mut resources = FactoryResources::new(&config).expect("Couldn't create resources.");
//     resources.redis.drop_all_dbs().unwrap();
//     let scenario_maker = SimpleDating {};
//     let result = scenario_maker.setup_scenario(&resources);
//     trace!("scenario result: {:?}", result);
//     let predicate_graph = InferenceGraph::new_mutable(&resources).unwrap();

//     let exciting = constant(Domain::Verb, "exciting".to_string());
//     let lonely = constant(Domain::Verb, "lonely".to_string());
//     let like = constant(Domain::Verb, "like".to_string());
//     let date = constant(Domain::Verb, "date".to_string());
//     let xjack = variable(Domain::Jack);
//     let xjill = variable(Domain::Jill);

//     let predicate = predicate(vec![
//         sub(xjack.clone()),
//         relation(like.clone()),
//         obj(xjill.clone()),
//     ]);

//     let result = predicate_graph.predicate_forward_links(&predicate).unwrap();
//     info!("{:?}", &result);
// }

#[test]
fn test_get_predicate_backward_links() {
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

    let like = constant(Domain::Verb, "like".to_string());
    let xjack = variable(Domain::Jack);
    let xjill = variable(Domain::Jill);

    let predicate = predicate(vec![
        sub(xjack.clone()),
        relation(like.clone()),
        obj(xjill.clone()),
    ]);

    let result = predicate_graph.predicate_backward_links(&predicate).unwrap();
    println!("{:?}", &result);
}

// #[test]
// fn test_get_conjoined_forward_links() {
//     let config = ConfigurationOptions {
//         entities_per_domain: 12,
//         print_training_loss: false,
//     };
//     let mut resources = FactoryResources::new(&config).expect("Couldn't create resources.");
//     resources.redis.drop_all_dbs().unwrap();
//     let scenario_maker = SimpleDating {};
//     let result = scenario_maker.setup_scenario(&resources);
//     trace!("scenario result: {:?}", result);
//     let predicate_graph = InferenceGraph::new_mutable(&resources).unwrap();

//     let exciting = constant(Domain::Verb, "exciting".to_string());
//     let lonely = constant(Domain::Verb, "lonely".to_string());
//     let like = constant(Domain::Verb, "like".to_string());
//     let date = constant(Domain::Verb, "date".to_string());
//     let xjack = variable(Domain::Jack);
//     let xjill = variable(Domain::Jill);

//     let conjoined = conjunction(vec![
//         predicate(vec![
//             sub(xjill.clone()),
//             relation(like.clone()),
//             obj(xjack.clone()),
//         ]),
//         predicate(vec![
//             sub(xjack.clone()),
//             relation(like.clone()),
//             obj(xjill.clone()),
//         ]),
//     ]);

//     let result = predicate_graph.conjoined_forward_links(&conjoined).unwrap();
//     info!("{:?}", &result);
// }

// #[test]
// fn test_get_conjoined_backward_links() {
//     let config = ConfigurationOptions {
//         entities_per_domain: 12,
//         print_training_loss: false,
//     };
//     let mut resources = FactoryResources::new(&config).expect("Couldn't create resources.");
//     resources.redis.drop_all_dbs().unwrap();
//     let scenario_maker = SimpleDating {};
//     let result = scenario_maker.setup_scenario(&resources);
//     trace!("scenario result: {:?}", result);
//     let predicate_graph = InferenceGraph::new_mutable(&resources).unwrap();

//     let exciting = constant(Domain::Verb, "exciting".to_string());
//     let lonely = constant(Domain::Verb, "lonely".to_string());
//     let like = constant(Domain::Verb, "like".to_string());
//     let date = constant(Domain::Verb, "date".to_string());
//     let xjack = variable(Domain::Jack);
//     let xjill = variable(Domain::Jill);

//     let conjoined = conjunction(vec![
//         predicate(vec![
//             sub(xjill.clone()),
//             relation(like.clone()),
//             obj(xjack.clone()),
//         ]),
//         predicate(vec![
//             sub(xjack.clone()),
//             relation(like.clone()),
//             obj(xjill.clone()),
//         ]),
//     ]);

//     let result = predicate_graph.conjoined_backward_links(&conjoined).unwrap();
//     info!("{:?}", &result);
// }