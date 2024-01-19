use std::error::Error;

use crate::{common::{graph::InferenceGraph, proposition_db::RedisFactDB, train::TrainingPlan, model::InferenceModel}, model::{exponential::ExponentialModel}, inference::inference::inference_compute_marginals};

use super::{resources::FactoryResources, setup::ConfigurationOptions};

pub fn inference_example(
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
    let proposition = &test_questions[config.test_example.unwrap() as usize];
    info!("testing proposition {:?}", &proposition.hash_string());
    inference_compute_marginals(model.clone(), proposition).unwrap();
    Ok(())
}

pub fn summarize_examples(
    config:&ConfigurationOptions,
    resources: &FactoryResources,
) -> Result<(), Box<dyn Error>> {
    let plan = TrainingPlan::new(&resources.redis)?;
    let graphical_model = InferenceModel::new_shared(&resources)?;
    let plan = TrainingPlan::new(&resources.redis)?;
    let model = InferenceModel::new_shared(&resources).unwrap();
    // test
    let test_questions = plan.get_test_questions().unwrap();
    for (index, proposition) in test_questions.iter().enumerate() {
        info!("testing proposition {:?}", &proposition.hash_string());
    }
    Ok(())
}
