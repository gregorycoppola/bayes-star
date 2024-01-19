use std::error::Error;

use crate::{common::{graph::InferenceGraph, proposition_db::RedisFactDB, train::TrainingPlan, model::InferenceModel}, model::{exponential::ExponentialModel}, inference::{inference::{inference_compute_marginals, Inferencer}, graph::PropositionGraph}, print_yellow};

use super::{resources::FactoryResources, setup::ConfigurationOptions};

pub fn interactive_inference_example(
    config: &ConfigurationOptions,
    resources: &FactoryResources,
) -> Result<(), Box<dyn Error>> {
    let plan = TrainingPlan::new(&resources.redis)?;
    let graphical_model = InferenceModel::new_shared(&resources)?;
    info!("do_training - Getting all implications");
    let plan = TrainingPlan::new(&resources.redis)?;
    let model = InferenceModel::new_shared(&resources).unwrap();
    // test
    let test_questions = plan.get_test_questions().unwrap();
    let target = &test_questions[config.test_example.unwrap() as usize];
    info!("testing proposition {:?}", &target.hash_string());
    let fact_memory = RedisFactDB::new_shared(&resources.redis)?;
    let proposition_graph = PropositionGraph::new_shared(model.graph.clone(), target)?;
    proposition_graph.visualize();
    let mut inferencer =
        Inferencer::new_mutable(model.clone(), proposition_graph.clone(), fact_memory)?;
    inferencer.initialize(target)?;
    inferencer.data.print_debug();
    print_yellow!("nodes {:?}", &proposition_graph.all_nodes);
    for (index, node) in proposition_graph.all_nodes.iter().enumerate() {
        print_yellow!("node {} {:?}", index, &node);
    }
    info!("done");
    Ok(())
}

pub fn summarize_examples(
    config:&ConfigurationOptions,
    resources: &FactoryResources,
) -> Result<(), Box<dyn Error>> {
    let plan = TrainingPlan::new(&resources.redis)?;
    let graphical_model = InferenceModel::new_shared(&resources)?;
    let model = InferenceModel::new_shared(&resources).unwrap();
    // test
    let test_questions = plan.get_test_questions().unwrap();
    for (index, proposition) in test_questions.iter().enumerate() {
        info!("testing proposition {:?}", &proposition.hash_string());
    }
    Ok(())
}
