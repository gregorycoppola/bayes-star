use bayes_star::{
    common::{interface::ScenarioMaker, redis::RedisManager, resources::FactoryResources, graph::PredicateGraph},
    scenarios::dating_simple::SimpleDating, inference::implications,
};
use log::info;
use bayes_star::model::config::ConfigurationOptions;

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
    let graph = PredicateGraph::new(&resources.redis).unwrap();
    let implications = graph.get_all_implications().unwrap();
    for implication in &implications {
        println!("implication {:?}", implication);
    }
}
