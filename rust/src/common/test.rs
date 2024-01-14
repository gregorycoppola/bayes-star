use std::error::Error;

use crate::{common::{graph::InferenceGraph, fact_db::RedisFactDB, train::TrainingPlan, model::GraphicalModel}, model::maxent::ExponentialModel};

use super::resources::FactoryResources;

pub fn test(
    resources: &FactoryResources,
) -> Result<(), Box<dyn Error>> {
    let plan = TrainingPlan::new(&resources.redis)?;
    let graphical_model = GraphicalModel::new_shared(&resources)?;
    info!("do_training - Getting all implications");
    Ok(())
}
