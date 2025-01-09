use bayes_star::common::{graph::InferenceGraph, resources::NamespaceBundle, setup::parse_configuration_options};

fn main() {
    let config: bayes_star::common::setup::CommandLineOptions = parse_configuration_options();
    let resources = NamespaceBundle::new(&config).unwrap();
    // let graph = InferenceGraph::new_shared(&resources).unwrap();
    let graph = InferenceGraph::new_shared(resources.connection.clone(), resources.namespace.clone()).unwrap();
    //
    // Domains.
    let all_domains = graph.get_all_domains().unwrap();
    println!("all_domains {:?}", &all_domains);
    for domain in &all_domains {
        let elements = graph.get_entities_in_domain(domain).unwrap();
        println!("elements: {:?}", &elements);
    }
    //
    // Relations.
    let all_relations = graph.get_all_relations().unwrap();
    println!("all_relations {:?}", &all_relations);
    for relation in &all_relations {
        println!("relation {:?}", relation);
    }
    //
    // Implications.
    let all_implications = graph.get_all_implications().unwrap();
    println!("all_implications {:?}", &all_implications);
    for implication in &all_implications {
        println!("implication {:?}", implication);
    }

    println!("main finishes");
}

