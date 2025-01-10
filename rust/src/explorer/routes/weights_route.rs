use redis::Connection;
use rocket::response::content::Html;

use crate::{common::{graph::InferenceGraph, resources::ResourceContext}, explorer::{diagram_utils::diagram_implication, render_utils::render_app_body}};

fn render_weights_part(connection: &mut Connection, graph: &InferenceGraph) -> String {
    let mut buffer = format!(
        r#"
        <div class='section_header'>
            Implication Factors
        </div>
    "#
    );
    let all_relations = graph.get_all_implications(connection).unwrap();
    println!("all_relations {:?}", &all_relations);
    for relation in &all_relations {
        buffer += &diagram_implication(relation);
    }
    buffer
}


pub fn internal_weights(experiment_name: &str, resources: &ResourceContext) -> Html<String> {
    let mut connection = resources.connection.lock().unwrap();
    let graph = InferenceGraph::new_mutable(experiment_name.to_string()).unwrap();
    let body_html = render_weights_part(&mut connection, &graph);
    let result = render_app_body(&body_html);
    Html(result.unwrap())
}