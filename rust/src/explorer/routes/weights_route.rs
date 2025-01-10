use redis::Connection;
use rocket::response::content::Html;

use crate::{common::{graph::InferenceGraph, resources::ResourceContext}, explorer::{diagram_utils::diagram_implication, render_utils::render_app_body}, model::{objects::ImplicationFactor, weights::{negative_feature, positive_feature, CLASS_LABELS}}};

fn render_one_weight_box(connection: &mut Connection, graph: &InferenceGraph, factor:&ImplicationFactor) -> String {
    let feature = factor.feature_string();
    let mut buffer = "".to_string();
    buffer += &format!("<div class='weight_box'>");
    for class_label in CLASS_LABELS {
        buffer += &format!("<div class='weight_box_row'>");
        let posf = positive_feature(&feature, class_label);
        let negf = negative_feature(&feature, class_label);
        buffer += &format!("</div>");
    }
    buffer += &format!("</div>");
    buffer
}

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