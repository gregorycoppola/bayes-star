use std::{error::Error, io};

use crate::{
    common::{
        graph::InferenceGraph, model::InferenceModel, proposition_db::RedisFactDB,
        train::TrainingPlan,
    },
    inference::{
        graph::PropositionGraph,
        inference::{inference_compute_marginals, Inferencer},
    },
    model::exponential::ExponentialModel,
    print_blue, print_green, print_red, print_yellow,
};

use super::{resources::FactoryResources, setup::ConfigurationOptions};

pub fn get_input_tokens_from_user() -> Vec<String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let trimmed = input.trim();
    let tokens: Vec<String> = trimmed.split_whitespace().map(|s| s.to_string()).collect();
    tokens
}

fn print_ordering(proposition_graph:&PropositionGraph) {
    let bfs = proposition_graph.get_bfs_order();
    for (index, node) in bfs.iter().enumerate() {
        if node.is_single() {
            info!("node {} {:?}", index, &node);
        } else {
            print_green!("node {} {:?}", index, &node);
        }
    }
}

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
    info!("done");
    loop {
        print_ordering(&proposition_graph);
        let tokens = get_input_tokens_from_user();
        println!("tokens {:?}", tokens);
        let function = &tokens[0];
        match function.as_str() {
            "set" => println!("Found 'hello'"),
            "quit" => break,
            _ => println!("Command not recognized."),
        };
    }
    Ok(())
}

pub fn summarize_examples(
    config: &ConfigurationOptions,
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
