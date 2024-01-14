use bayes_star::common::setup::parse_configuration_options;
use bayes_star::common::resources::FactoryResources;

#[macro_use]
extern crate log;

fn main() {
    let config = parse_configuration_options();
    let _resources = FactoryResources::new(&config).expect("Couldn't create resources.");
    info!("TODO: implement test");
    warn!("program done");
}
