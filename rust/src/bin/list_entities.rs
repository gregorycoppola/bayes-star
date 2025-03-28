use bayes_star::common::{graph::InferenceGraph, resources::ResourceContext, setup::parse_configuration_options};

fn main() {
    let config: bayes_star::common::setup::CommandLineOptions = parse_configuration_options();
    let resources = ResourceContext::new(&config).unwrap();
    let mut connection = resources.connection.lock().unwrap();
    let graph = InferenceGraph::new_shared(config.scenario_name.clone()).unwrap();
    //
    // Domains.
    let all_domains = graph.get_all_domains(&mut connection).unwrap();
    println!("all_domains {:?}", &all_domains);
    for domain in &all_domains {
        let elements = graph.get_entities_in_domain(&mut connection, domain).unwrap();
        println!("elements: {:?}", &elements);
    }
    //
    // Relations.
    let all_relations = graph.get_all_relations(&mut connection).unwrap();
    println!("all_relations {:?}", &all_relations);
    for relation in &all_relations {
        println!("relation {:?}", relation);
    }
    //
    // Implications.
    let all_implications = graph.get_all_implications(&mut connection).unwrap();
    println!("all_implications {:?}", &all_implications);
    for implication in &all_implications {
        println!("implication {:?}", implication);
    }

    println!("main finishes");
}

