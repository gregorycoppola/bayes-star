use std::error::Error;

use redis::Connection;
use rocket::response::content::Html;

use crate::{
    common::{
        graph::InferenceGraph, model::InferenceModel, proposition_db::EmptyBeliefTable,
        resources::ResourceContext,
    },
    explorer::{diagram_utils::{diagram_implication, diagram_predicate, diagram_proposition}, render_utils::render_app_body},
    inference::{graph::PropositionGraph, inference::Inferencer},
};

fn iterate_through_factors(
    scenario_name: &str,
    test_scenario: &str,
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
        let proposition = single_node.extract_single();
        buffer += &diagram_proposition(&proposition);
    }
    Ok(buffer)
}

fn render_implication_part(connection: &mut Connection, graph: &InferenceGraph) -> String {
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

pub fn internal_factors(experiment_name: &str, resource_context: &ResourceContext) -> Html<String> {
    let mut connection = resource_context.connection.lock().unwrap();
    let graph = InferenceGraph::new_mutable(experiment_name.to_string()).unwrap();
    let body_html = render_implication_part(&mut connection, &graph);
    let result = render_app_body(&body_html);
    Html(result.unwrap())
}
