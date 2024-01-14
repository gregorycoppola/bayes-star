use std::error::Error;

use crate::{common::{graph::InferenceGraph, proposition_db::RedisFactDB, train::TrainingPlan, model::GraphicalModel}, model::{maxent::ExponentialModel}, inference::inference::inference_compute_marginals};

use super::resources::FactoryResources;

pub fn test(
    resources: &FactoryResources,
) -> Result<(), Box<dyn Error>> {
    let plan = TrainingPlan::new(&resources.redis)?;
    let graphical_model = GraphicalModel::new_shared(&resources)?;
    info!("do_training - Getting all implications");
    let plan = TrainingPlan::new(&resources.redis)?;
    let model = GraphicalModel::new_shared(&resources).unwrap();
    // test
    let test_questions = plan.get_test_questions().unwrap();
    for proposition in &test_questions {
        info!("testing proposition {:?}", &proposition);
        inference_compute_marginals(model.clone(), proposition).unwrap();
    }
    info!("TODO: implement test");
    Ok(())
}
