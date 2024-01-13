use std::collections::HashMap;

use bayes_star::inference::graph::PropositionGraph;
use bayes_star::model::config::ConfigurationOptions;
use bayes_star::model::objects::Proposition;
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

    let exciting = constant(Domain::Verb, "exciting".to_string());
    let lonely = constant(Domain::Verb, "lonely".to_string());
    let like = constant(Domain::Verb, "like".to_string());
    let date = constant(Domain::Verb, "date".to_string());

    let jack1 = constant(Domain::Jack, "jack1".to_string());
    let jill1 = constant(Domain::Jill, "jill1".to_string());

    let predicate = predicate(vec![
        subject(jack1.clone()),
        relation(like.clone()),
        object(jill1.clone()),
    ]);
    let proposition = Proposition::from(predicate);

    let predicate_graph = InferenceGraph::new_shared(&resources).unwrap();
    let proposition_graph = PropositionGraph::new_shared(predicate_graph.clone(), todo!()).unwrap();
    let result = proposition_graph.proposition_backward_links(&proposition).unwrap();
    println!("{:?}", &result);
}
