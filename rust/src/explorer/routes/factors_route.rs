use std::error::Error;

use redis::Connection;
use rocket::response::content::Html;

use crate::{
    common::{
        graph::InferenceGraph, model::InferenceModel, proposition_db::EmptyBeliefTable,
        resources::ResourceContext,
    },
    explorer::{
        diagram_utils::{
            diagram_implication, diagram_predicate, diagram_proposition, diagram_proposition_group,
        },
        render_utils::render_app_body,
    },
    inference::{
        graph::PropositionGraph,
        inference::{compute_each_combination, Inferencer},
        table::{PropositionNode, VariableAssignment},
    },
    model::objects::Proposition,
};

pub fn diagram_variable_assignment(assignment: &VariableAssignment) -> String {
    let mut html =
        String::from("<table border='1'><tr><th>PropositionNode</th><th>Value</th></tr>");
    let sorted_keys: Vec<_> = assignment.assignment_map.iter().collect();
    for (key, value) in sorted_keys {
        let row = format!("<tr><td>{:?}</td><td>{}</td></tr>", key, value);
        html.push_str(&row);
    }
    html.push_str("</table>");
    html
}

fn graph_full_factor(inferencer: &Inferencer, target: &Proposition) -> String {
    let node = &PropositionNode::from_single(target);
    let mut buffer = "".to_string();
    buffer += &format!("<div class='factor_box'>");
    buffer += &diagram_proposition(target);
    let parent_nodes = inferencer.proposition_graph.get_all_backward(node);
    buffer += &format!("<div class='factor_parent_box'>");
    for parent_node in &parent_nodes {
        let proposition = parent_node.extract_group();
        buffer += &diagram_proposition_group(&proposition);
    }
    buffer += &format!("</div>");
    buffer += &format!("</div>");
    buffer
}

fn iterate_through_factors(
    scenario_name: &str,
    resource_context: &ResourceContext,
) -> Result<String, Box<dyn Error>> {
    let model = InferenceModel::new_shared(scenario_name.to_string()).unwrap();
    let fact_memory = EmptyBeliefTable::new_shared(scenario_name)?;
    let mut connection = resource_context.connection.lock().unwrap();
    let target = model.graph.get_target(&mut connection)?;
    let proposition_graph = PropositionGraph::new_shared(&mut connection, &model.graph, target)?;
    let inferencer =
        Inferencer::new_mutable(model.clone(), proposition_graph.clone(), fact_memory)?;
    let mut buffer = "".to_string();
    for single_node in &inferencer.bfs_order {
        if single_node.is_single() {
            let proposition = single_node.extract_single();
            buffer += &graph_full_factor(&inferencer, &proposition);
        }
    }
    Ok(buffer)
}

pub fn internal_factors(experiment_name: &str, resource_context: &ResourceContext) -> Html<String> {
    let graph = InferenceGraph::new_mutable(experiment_name.to_string()).unwrap();
    let body_html = iterate_through_factors(experiment_name, resource_context).unwrap();
    let result = render_app_body(&body_html);
    Html(result.unwrap())
}
