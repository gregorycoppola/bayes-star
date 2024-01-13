use super::graph::PredicateGraph;
use super::interface::{FactDB, ScenarioMaker};
use super::model::FactorModel;
use super::resources::FactoryResources;
use super::train::TrainingPlan;
use crate::common::fact_db::RedisFactDB;
use crate::common::model::GraphicalModel;
use crate::common::redis::RedisManager;
use crate::model::choose::{
    extract_backimplications_from_proposition, extract_factor_context_for_proposition,
};
use crate::model::maxent::ExponentialModel;
use std::borrow::BorrowMut;
use std::error::Error;

pub fn do_training(resources: &FactoryResources) -> Result<(), Box<dyn Error>> {
    let graph = PredicateGraph::new(&resources.redis)?;
    let fact_db = RedisFactDB::new(&resources.redis)?;
    let plan = TrainingPlan::new(&resources.redis)?;
    let mut factor_model = ExponentialModel::new(&resources)?;
    trace!("do_training - Getting all implications");
    let implications = graph.get_all_implications()?;
    for implication in implications {
        trace!("do_training - Processing implication: {:?}", implication);
        factor_model.initialize_connection(&implication)?;
    }
    trace!("do_training - Getting all propositions");
    let propositions = plan.get_training_questions()?;
    trace!(
        "do_training - Processing propositions: {}",
        propositions.len()
    );
    let mut examples_processed = 0;
    for proposition in &propositions {
        trace!("do_training - Processing proposition: {:?}", proposition);
        let factor = extract_factor_context_for_proposition(&fact_db, &graph, proposition.clone())?;
        trace!("do_training - Backimplications: {:?}", &factor);
        let probabiity_opt = fact_db.get_proposition_probability(proposition)?;
        let probability = probabiity_opt.expect("Probability should exist.");
        match factor_model.train(&factor, probability) {
            Ok(_) => trace!(
                "do_training - Successfully trained on proposition: {:?}",
                proposition
            ),
            Err(e) => {
                panic!(
                    "do_training - Error in train_on_example for proposition {} {:?}: {:?}",
                    examples_processed, proposition, e
                )
            }
        }
        examples_processed += 1;
    }
    trace!(
        "do_training - Training complete: examples processed {}",
        examples_processed
    );
    Ok(())
}

pub fn setup_and_train(
    resources: &FactoryResources,
    scenario_maker: &dyn ScenarioMaker,
) -> Result<(), Box<dyn Error>> {
    let mut redis_client = RedisManager::new()?;
    redis_client.drop_all_dbs()?;
    let model_spec = "dummy_model_spec".to_string();
    let result = scenario_maker.setup_scenario(resources);
    info!("scenario result: {:?}", result);
    let train_result = do_training(resources);
    info!("train result: {:?}", train_result);
    Ok(())
}
