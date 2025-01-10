use std::{error::Error, rc::Rc};

use redis::Connection;
use rocket::response::content::Html;

use crate::{
    common::{
        graph::InferenceGraph, model::InferenceModel, proposition_db::EmptyBeliefTable,
        resources::ResourceContext, setup::CommandLineOptions, train::TrainingPlan,
    },
    explorer::{
        diagram_utils::{diagram_implication, diagram_predicate, diagram_proposition_factor},
        render_utils::render_app_body,
    },
    inference::{graph::PropositionGraph, inference::Inferencer, table::PropositionNode},
    model::{
        choose::extract_backimplications_from_proposition,
        objects::{Proposition, PropositionGroup},
    },
};

fn backwards_print_group(
    connection: &mut Connection,
    inferencer: &Inferencer,
    target: &PropositionGroup,
) -> Result<String, Box<dyn Error>> {
    let proposition_node = PropositionNode::from_group(&target);
    let backlinks = inferencer
        .proposition_graph
        .get_all_backward(&proposition_node);
    let mut buffer = "".to_string();
    for backlink in &backlinks {
        let single = backlink.extract_single();
        let part = backwards_print_single(connection, inferencer, &single)?;
        buffer += &part;
    }
    Ok(buffer)
}

fn backwards_print_single(
    connection: &mut Connection,
    inferencer: &Inferencer,
    target: &Proposition,
) -> Result<String, Box<dyn Error>> {
    let proposition_node = PropositionNode::from_single(&target);
    let backlinks = inferencer
        .proposition_graph
        .get_all_backward(&proposition_node);
    let mut buffer = "".to_string();
    for backlink in &backlinks {
        let group = backlink.extract_group();
        let part = backwards_print_group(connection, inferencer, &group)?;
        buffer += &part;
    }
    buffer += &format!(
        r#"
        <div class='network_cell'>
            {target_part}
        </div>
    "#,
        target_part = diagram_predicate(&target.predicate)
    );
    let backimplications =
        extract_backimplications_from_proposition(connection, &inferencer.model.graph, target)
            .unwrap();
    buffer += &format!(
        r#"
        <div class='network_row'>
    "#,
    );
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
    buffer += &format!(
        r#"
        </div>
    "#,
    );
    Ok(buffer)
}

fn render_network(bundle: &ResourceContext, namespace: &str) -> Result<String, Box<dyn Error>> {
    let graph = InferenceGraph::new_shared(namespace.to_string())?;
    let mut connection = bundle.connection.lock().unwrap();
    let target = graph.get_target(&mut connection)?;
    let proposition_graph = PropositionGraph::new_shared(&mut connection, &graph, target)?;
    proposition_graph.visualize();
    let model = InferenceModel::new_shared(namespace.to_string()).unwrap();
    let fact_memory = EmptyBeliefTable::new_shared(namespace)?;
    let inferencer =
        Inferencer::new_mutable(model.clone(), proposition_graph.clone(), fact_memory)?;
    let result = backwards_print_single(
        &mut connection,
        &inferencer,
        &inferencer.proposition_graph.target,
    )?;
    Ok(result)
}

pub fn internal_network(experiment_name: &str, namespace: &ResourceContext) -> Html<String> {
    let network = render_network(namespace, experiment_name).unwrap();
    let body_html = format!(
        r#"
        {network}
    "#,
    );
    let result = render_app_body(&body_html);
    Html(result.unwrap())
}
