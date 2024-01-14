use std::error::Error;

use crate::{common::{graph::InferenceGraph, fact_db::RedisFactDB, train::TrainingPlan}, model::maxent::ExponentialModel};

use super::resources::FactoryResources;

pub fn test(
    resources: &FactoryResources,
) -> Result<(), Box<dyn Error>> {
    let graph = InferenceGraph::new_mutable(resources)?;
    let fact_db = RedisFactDB::new_mutable(&resources.redis)?;
    let plan = TrainingPlan::new(&resources.redis)?;
    let factor_model = ExponentialModel::new_shared(&resources)?;
    info!("do_training - Getting all implications");
    Ok(())
}
