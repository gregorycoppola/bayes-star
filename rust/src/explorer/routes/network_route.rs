use std::{error::Error, rc::Rc};

use rocket::response::content::Html;

use crate::{common::{graph::InferenceGraph, model::InferenceModel, proposition_db::EmptyBeliefTable, resources::NamespaceBundle, setup::CommandLineOptions, train::TrainingPlan}, explorer::render_utils::render_app_body, inference::{graph::PropositionGraph, inference::Inferencer}};

fn get_resources() -> NamespaceBundle {
    todo!()
}

fn render_network(graph: &InferenceGraph, config: &CommandLineOptions) -> Result<String, Box<dyn Error>> {
    // let proposition_graph = PropositionGraph::new_shared(graph)?;
    // // proposition_graph.visualize();
    // let mut inferencer =
    //     Inferencer::new_mutable(config, model.clone(), proposition_graph.clone(), fact_memory)?;
    // inferencer.initialize_chart()?;
    Ok("todo".to_string())
}

pub fn internal_network(_experiment_name: &str, graph: &InferenceGraph, config: &CommandLineOptions) -> Html<String> {
    let network = render_network(graph, config).unwrap();
    let body_html = format!(
        r#"
        {network}
    "#,
    );
    let result = render_app_body(&body_html);
    Html(result.unwrap())
}
