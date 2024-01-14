use std::borrow::Borrow;

use bayes_star::common::setup::parse_configuration_options;
use bayes_star::common::{run::setup_and_train, resources::FactoryResources};
use bayes_star::scenarios::factory::ScenarioMakerFactory;

#[macro_use]
extern crate log;

fn main() {
    let config = parse_configuration_options();
    let resources = FactoryResources::new(&config).expect("Couldn't create resources.");
    let scenario_maker = ScenarioMakerFactory::new_shared(&"DatingSimple".to_string()).expect("Could not make scenario maker.");
    setup_and_train(&resources, scenario_maker.borrow()).expect("Error in training.");
    warn!("program done");
}
