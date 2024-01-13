use bayes_star::{scenarios::dating_prob2::SimpleDating, common::{interface::ScenarioMaker, redis::RedisManager}};
use log::info;


#[test]
fn test_store_entity() {
    let redis_client = RedisManager::new().unwrap();
    let scenario_maker = SimpleDating{};
    let result = scenario_maker.setup_scenario(
        &redis_client);
    info!("scenario result: {:?}", result);
}