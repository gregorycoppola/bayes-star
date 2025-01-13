use std::error::Error;

use redis::Connection;
use rocket::response::content::Html;

use crate::{
    common::{
        graph::InferenceGraph, model::InferenceModel, proposition_db::EmptyBeliefTable,
        resources::ResourceContext,
    },
    explorer::{
        diagram_utils::{diagram_predicate, diagram_proposition_factor},
        render_utils::render_app_body,
    },
    inference::{
        graph::PropositionGraph,
        inference::{Inferencer, MarginalTable},
        rounds::run_inference_rounds,
        table::PropositionNode,
    },
    model::{
        choose::extract_backimplications_from_proposition,
        objects::{Proposition, PropositionGroup},
    },
};

fn backwards_print_group(
    connection: &mut Connection,
    inferencer: &Inferencer,
    target: &PropositionGroup,
    table: &MarginalTable,
) -> Result<String, Box<dyn Error>> {
    let proposition_node = PropositionNode::from_group(&target);
    let backlinks = inferencer
        .proposition_graph
        .get_all_backward(&proposition_node);
    let mut buffer = "".to_string();
    for backlink in &backlinks {
        let single = backlink.extract_single();
        let part = backwards_print_single_with_marginal_table(connection, inferencer, &single, table)?;
        buffer += &part;
    }
    Ok(buffer)
}

fn backwards_print_single_with_marginal_table(
    connection: &mut Connection,
    inferencer: &Inferencer,
    target: &Proposition,
    table: &MarginalTable,
) -> Result<String, Box<dyn Error>> {
    let proposition_node = PropositionNode::from_single(&target);
    let backlinks = inferencer
        .proposition_graph
        .get_all_backward(&proposition_node);
    let mut buffer = "".to_string();
    buffer += &format!(r#" <div class='proof_box'> "#,);
    buffer += &format!(r#" <div class='network_row'> "#,);
    for backlink in &backlinks {
        let group = backlink.extract_group();
        let part = backwards_print_group(connection, inferencer, &group, table)?;
        buffer += &part;
    }
    buffer += &format!(r#" </div>"#,);
    let backimplications =
        extract_backimplications_from_proposition(connection, &inferencer.model.graph, target)
            .unwrap();
    buffer += &format!(r#" <div class='network_row'> "#,);
    for backimplication in &backimplications {
        buffer += &format!(
            r#"
            <span class='network_column'>
                {implication_part}
            </span>
        "#,
            implication_part = diagram_proposition_factor(backimplication)
        );
    }
    buffer += &format!(r#" </div> "#,);
    buffer += &format!(
        r#"
        <div class='network_row'>
            {target_part}
        </div>
    "#,
        target_part = diagram_predicate(&target.predicate)
    );
    buffer += &format!(r#" </div> "#,); // "proof_box"
    Ok(buffer)
}

fn safe_network_animations(
    connection: &mut Connection,
    namespace: &str,
    marginal_tables: &Vec<MarginalTable>,
) -> Result<String, Box<dyn Error>> {
    let graph = InferenceGraph::new_shared(namespace.to_string())?;
    let target = graph.get_target(connection)?;
    let proposition_graph = PropositionGraph::new_shared(connection, &graph, target)?;
    proposition_graph.visualize();
    let model = InferenceModel::new_shared(namespace.to_string()).unwrap();
    let fact_memory = EmptyBeliefTable::new_shared(namespace)?;
    let inferencer =
        Inferencer::new_mutable(model.clone(), proposition_graph.clone(), fact_memory)?;
    let mut result = "".to_string();
    for table in marginal_tables {
        result += &backwards_print_single_with_marginal_table(
            connection,
            &inferencer,
            &inferencer.proposition_graph.target,
            table,
        )?;
    }
    Ok(result)
}

pub fn internal_animation(
    experiment_name: &str,
    test_scenario: &str,
    resource_context: &ResourceContext,
) -> Html<String> {
    let mut connection = resource_context.connection.lock().unwrap();
    let marginal_tables = run_inference_rounds(&mut connection, experiment_name, test_scenario)
        .expect("Testing failed.");
    let body_html = safe_network_animations(&mut connection, experiment_name, &marginal_tables).unwrap();
    let result = render_app_body(&body_html);
    Html(result.unwrap())
}
