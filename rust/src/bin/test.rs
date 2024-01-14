use bayes_star::common::model::GraphicalModel;
use bayes_star::common::setup::parse_configuration_options;
use bayes_star::common::resources::FactoryResources;

#[macro_use]
extern crate log;

fn main() {
    let config = parse_configuration_options();
    let resources = FactoryResources::new(&config).expect("Couldn't create resources.");
    let model = GraphicalModel::new_shared(&resources).unwrap();
    info!("TODO: implement test");
    warn!("program done");
}
