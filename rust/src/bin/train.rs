use std::borrow::Borrow;

use bayes_star::common::setup::parse_configuration_options;
use bayes_star::common::{resources::ResourceContext, train::setup_and_train};
use bayes_star::scenarios::factory::ScenarioMakerFactory;

#[macro_use]
extern crate log;

fn main() {
    let config = parse_configuration_options();
    let resources = ResourceContext::new(&config).expect("Couldn't create resources.");
    let scenario_maker = ScenarioMakerFactory::new_shared(&config.scenario_name).unwrap();
    setup_and_train(&resources, scenario_maker.borrow(), &config.scenario_name).expect("Error in training.");
    trace!("program done");
}
