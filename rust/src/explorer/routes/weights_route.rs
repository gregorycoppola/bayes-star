use redis::Connection;
use rocket::response::content::Html;

use crate::{
    common::{graph::InferenceGraph, resources::ResourceContext},
    explorer::{diagram_utils::diagram_implication, render_utils::render_app_body},
    model::{
        objects::ImplicationFactor,
        weights::{negative_feature, positive_feature, ExponentialWeights, CLASS_LABELS},
    },
};

fn render_one_weight_box(
    connection: &mut Connection,
    graph: &InferenceGraph,
    factor: &ImplicationFactor,
) -> String {
    let weights = ExponentialWeights::new(graph.namespace.clone()).unwrap();
    let feature = factor.unique_key();
    let mut buffer = "".to_string();
    buffer += &format!("<div class='weight_box'>");
    buffer += &format!(
        r#"
        <div class='weight_box_row'>
            <div class='weight_box_cell'>
            </div>
            <div class='weight_box_cell'>
                false
            </div>
            <div class='weight_box_cell'>
                true
            </div>
        </div>
    "#
    );
    for class_label in CLASS_LABELS {
        let posf = positive_feature(&feature, class_label);
        let negf = negative_feature(&feature, class_label);
        let posf_count = weights.read_single_weight(connection, &posf).unwrap();
        let negf_count = weights.read_single_weight(connection, &negf).unwrap();
        let posf_css = if posf_count > 0.1f64 {
            "positive_weight".to_string()
        } else if posf_count < -0.1f64 {
            "negative_weight".to_string()
        } else {
            "neutral_weight".to_string()
        };
        let negf_css = if negf_count > 0.1f64 {
            "positive_weight".to_string()
        } else if negf_count < -0.1f64 {
            "negative_weight".to_string()
        } else {
            "neutral_weight".to_string()
        };
        buffer += &format!(
            r#"
            <div class='weight_box_row'>
                <div class='weight_box_cell'>
                    {class_label}
                </div>
                <div class='weight_box_cell {negf_css}'>
                    {negf_count}
                </div>
                <div class='weight_box_cell {posf_css}'>
                    {posf_count}
                </div>
            </div>
        "#
        );
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
        buffer += &render_one_weight_box(connection, graph, relation);
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
