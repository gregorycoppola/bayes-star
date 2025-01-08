use rocket::response::content::Html;

use crate::{common::graph::InferenceGraph, explorer::render::render_app_body};


fn render_experiment_parts(graph: &InferenceGraph) -> String {
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

    "".to_string()
}

pub fn internal_experiment(experiment_name: &str) -> Html<String> {
    let result = render_app_body(experiment_name);
    Html(result.unwrap())
}