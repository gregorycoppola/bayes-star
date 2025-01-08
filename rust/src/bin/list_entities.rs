use bayes_star::common::{graph::InferenceGraph, resources::FactoryResources, setup::parse_configuration_options};

fn main() {
    let config: bayes_star::common::setup::ConfigurationOptions = parse_configuration_options();
    let resources = FactoryResources::new(&config).expect("Couldn't create resources.");
    let graph = InferenceGraph::new_shared(&resources).expect("");
    let all_domains = graph.get_all_domains().unwrap();
    println!("all_domains {:?}", &all_domains);

    for domain in &all_domains {
        let elements = graph.get_entities_in_domain(domain).unwrap();
        println!("elements: {:?}", &elements);
    }
    println!("main finishes");
}

