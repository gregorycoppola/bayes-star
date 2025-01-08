use std::error::Error;

use rocket::response::content::Html;

use crate::{common::{graph::InferenceGraph, model::InferenceModel, proposition_db::EmptyBeliefTable, resources::FactoryResources, train::TrainingPlan}, explorer::render_utils::render_app_body, inference::{graph::PropositionGraph, inference::Inferencer}};

fn get_resources() -> FactoryResources {
    todo!()
}

fn render_network(graph: &InferenceGraph) -> Result<String, Box<dyn Error>> {
    let resources = get_resources();
    let plan = TrainingPlan::new(&resources)?;
    let model = InferenceModel::new_shared(&resources).unwrap();
    let test_questions = plan.get_test_questions().unwrap();
    let target = &test_questions[0];
    let fact_memory = EmptyBeliefTable::new_shared(&resources.redis)?;
    let proposition_graph = PropositionGraph::new_shared(model.graph.clone(), target)?;
    proposition_graph.visualize();
    let mut inferencer =
        Inferencer::new_mutable(&config, model.clone(), proposition_graph.clone(), fact_memory)?;
    inferencer.initialize_chart()?;
    Ok("todo".to_string())
}

pub fn internal_network(graph: &InferenceGraph) -> Html<String> {
    let network = render_network(graph).unwrap();
    let body_html = format!(
        r#"
        {network}
    "#,
    );
    let result = render_app_body(&body_html);
    Html(result.unwrap())
}
