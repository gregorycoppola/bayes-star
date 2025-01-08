use bayes_star::common::{graph::InferenceGraph, resources::FactoryResources, setup::parse_configuration_options};

fn main() {
    let config: bayes_star::common::setup::ConfigurationOptions = parse_configuration_options();
    let resources = FactoryResources::new(&config).expect("Couldn't create resources.");
    let mut graph = InferenceGraph::new_mutable(&resources).expect("");
    println!("main finishes");
}

