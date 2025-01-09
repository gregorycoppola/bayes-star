use std::{error::Error, rc::Rc};

use rocket::response::content::Html;

use crate::{common::{graph::InferenceGraph, model::InferenceModel, proposition_db::EmptyBeliefTable, resources::NamespaceBundle, setup::CommandLineOptions, train::TrainingPlan}, explorer::render_utils::render_app_body, inference::{graph::PropositionGraph, inference::Inferencer}};

fn get_resources() -> NamespaceBundle {
    todo!()
}

fn render_network(namespace: &NamespaceBundle) -> Result<String, Box<dyn Error>> {
    let graph = InferenceGraph::new_shared(namespace.connection.clone(), namespace.namespace.clone())?;
    let proposition_graph = PropositionGraph::new_shared(&graph)?;
    // // proposition_graph.visualize();
    // let mut inferencer =
    //     Inferencer::new_mutable(config, model.clone(), proposition_graph.clone(), fact_memory)?;
    // inferencer.initialize_chart()?;
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
