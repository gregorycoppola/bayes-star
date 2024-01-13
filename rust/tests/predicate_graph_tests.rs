use bayes_star::{
    common::{interface::ScenarioMaker, redis::RedisManager, resources::FactoryResources},
    scenarios::dating_simple::SimpleDating,
};
use log::info;
use bayes_star::model::config::ConfigurationOptions;

#[test]
fn test_store_entity() {
    let config = ConfigurationOptions {
        entities_per_domain: 12,
        print_training_loss: false,
    };
    let resources = FactoryResources::new(&config).expect("Couldn't create resources.");
    let scenario_maker = SimpleDating {};
    let result = scenario_maker.setup_scenario(&resources);
    info!("scenario result: {:?}", result);
}
