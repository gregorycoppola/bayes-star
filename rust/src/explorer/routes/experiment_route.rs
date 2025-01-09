use redis::Connection;
use rocket::response::content::Html;

use crate::{common::{graph::InferenceGraph, redis::seq_push, resources::NamespaceBundle}, explorer::{diagram_utils::diagram_implication, render_utils::render_app_body}};

fn render_domain_part(graph: &InferenceGraph) -> String {
    let mut buffer = format!(
        r#"
        <div class='section_header'>
            Domains
        </div>
    "#
    );
    let all_domains = graph.get_all_domains().unwrap();
    println!("all_domains {:?}", &all_domains);
    for domain in &all_domains {
        let elements = graph.get_entities_in_domain(domain).unwrap();
        println!("elements: {:?}", &elements);
        buffer += &format!(
            r#"
                <div class='row_element'>
                    <span class='domain_label'>{domain}</span>
                    <span><img src='/static/images/domains/{domain}.png' class='domain_icon'></img></span>
                </div>
            "#,
        )
    }
    buffer
}

fn render_relation_part(graph: &InferenceGraph) -> String {
    let mut buffer = format!(
        r#"
        <div class='section_header'>
            Relations
        </div>
    "#
    );
    let all_relations = graph.get_all_relations().unwrap();
    println!("all_relations {:?}", &all_relations);
    for relation in &all_relations {
        println!("relation {:?}", relation);
        buffer += &format!(r#" <div class='row_element'>"#);
        buffer += &format!(
            r#" <span class='relation_name'>{relation_name}</span>"#,
            relation_name = &relation.relation_name
        );
        for argument_type in &relation.types {
            buffer += &format!(
                r#"
                        <span class='domain_label'>{domain_name}</span>
                        <span><img src='/static/images/domains/{domain_name}.png' class='domain_icon'></img></span>
                "#,
                domain_name = argument_type.domain
            );
        }
        buffer += &format!(r#"</div>"#)
    }
    buffer
}

fn render_implication_part(graph: &InferenceGraph) -> String {
    let mut buffer = format!(
        r#"
        <div class='section_header'>
            Implication Factors
        </div>
    "#
    );
    let all_relations = graph.get_all_implications().unwrap();
    println!("all_relations {:?}", &all_relations);
    for relation in &all_relations {
        buffer += &diagram_implication(relation);
    }
    buffer
}

fn render_experiment_parts(graph: &InferenceGraph) -> String {
    format!(
        r#"
        {domain_part}
        {relation_part}
        {implication_part}
    "#,
        domain_part = render_domain_part(graph),
        relation_part = render_relation_part(graph),
        implication_part = render_implication_part(graph),
    )
}

fn render_experiment_name(experiment_name: &str) -> String {
    format!(
        r#"
        <div class='section_header'>
            Experiment
        </div>
        <div class='experiment_name'>
            {experiment_name}
        </div>
    "#
    )
}

pub fn internal_experiment(experiment_name: &str, resources: &NamespaceBundle) -> Html<String> {
    let graph = InferenceGraph::new_mutable(resources.connection.clone(), resources.namespace.clone()).unwrap();
    // let graph = InferenceGraph::new_mutable(redis_connection, namespace)
    let body_html = format!(
        r#"
        {name_part}
        {main_part}
    "#,
        name_part = render_experiment_name(experiment_name),
        main_part = render_experiment_parts(&graph),
    );
    let result = render_app_body(&body_html);
    Html(result.unwrap())
}
