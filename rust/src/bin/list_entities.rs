use bayes_star::common::{resources::FactoryResources, setup::parse_configuration_options};

fn main() {
    let config: bayes_star::common::setup::ConfigurationOptions = parse_configuration_options();
    let resources = FactoryResources::new(&config).expect("Couldn't create resources.");
    println!("main finishes");
}

