use std::{error::Error, rc::Rc};

use rocket::response::content::Html;

use crate::{common::{graph::InferenceGraph, model::InferenceModel, proposition_db::EmptyBeliefTable, resources::NamespaceBundle, setup::CommandLineOptions, train::TrainingPlan}, explorer::render_utils::render_app_body, inference::{graph::PropositionGraph, inference::Inferencer, table::PropositionNode}, model::{choose::extract_backimplications_from_proposition, objects::{Proposition, PropositionGroup}}};

fn get_resources() -> NamespaceBundle {
    todo!()
}

fn backwards_print_group(inferencer: &Inferencer, target: &PropositionGroup) -> String {
    let proposition_node = PropositionNode::from_group(&target);
    let backlinks = inferencer.proposition_graph.get_all_backward(&proposition_node);
    for backlink in &backlinks {
        let single = backlink.extract_single();
        backwards_print_single(inferencer, &single);
    }
    "".to_string()
}

fn backwards_print_single(inferencer: &Inferencer, target: &Proposition) -> String {
    let proposition_node = PropositionNode::from_single(&target);
    let backlinks = inferencer.proposition_graph.get_all_backward(&proposition_node);
    for backlink in &backlinks {
        let group = backlink.extract_group();
        backwards_print_group(inferencer, &group);
    }

    let backimplications = extract_backimplications_from_proposition(&inferencer.model.graph, target).unwrap();
    "".to_string()
}

fn render_network(namespace: &NamespaceBundle) -> Result<String, Box<dyn Error>> {
    let graph = InferenceGraph::new_shared(namespace.connection.clone(), namespace.namespace.clone())?;
    let proposition_graph = PropositionGraph::new_shared(&graph)?;
    proposition_graph.visualize();
    let model = InferenceModel::new_shared(namespace).unwrap();
    let fact_memory = EmptyBeliefTable::new_shared(namespace)?;
    let mut inferencer =
        Inferencer::new_mutable(model.clone(), proposition_graph.clone(), fact_memory)?;
    inferencer.initialize_chart()?;
    inferencer.do_full_forward_and_backward()?;
    Ok("todo".to_string())
}

pub fn internal_network(experiment_name: &str, namespace: &NamespaceBundle) -> Html<String> {
    let network = render_network(namespace).unwrap();
    let body_html = format!(
        r#"
        {network}
    "#,
    );
    let result = render_app_body(&body_html);
    Html(result.unwrap())
}
