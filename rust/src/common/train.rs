use crate::{
    common::{interface::BeliefTable, redis::seq_get_all}, model::{
        self,
        exponential::ExponentialModel,
        objects::{
            Domain, Entity, PredicateFactor, Predicate, PredicateGroup,
            Proposition, PropositionGroup,
        },
    }, print_yellow
};
use redis::{Commands, Connection};
use serde::Deserialize;
use std::{cell::RefCell, error::Error};

use super::{
    interface::{PredictStatistics, TrainStatistics},
    redis::RedisManager, model::FactorContext,
};
use super::graph::InferenceGraph;
use super::interface::ScenarioMaker;
use super::model::FactorModel;
use super::resources::FactoryResources;
use crate::common::proposition_db::RedisBeliefTable;
use crate::common::model::InferenceModel;
use crate::model::choose::extract_backimplications_from_proposition;
use std::borrow::BorrowMut;



pub struct TrainingPlan {
    redis_connection: RefCell<redis::Connection>,
}

impl TrainingPlan {
    pub fn new(redis: &RedisManager) -> Result<Self, Box<dyn Error>> {
        let redis_connection = redis.get_connection()?;
        Ok(TrainingPlan { redis_connection })
    }

    pub fn add_proposition_to_queue(
        &mut self,
        queue_name: &String,
        proposition: &Proposition,
    ) -> Result<(), Box<dyn Error>> {
        trace!(
            "GraphicalModel::add_to_training_queue - Start. Input proposition: {:?}",
            proposition
        );
        let serialized_proposition = match serde_json::to_string(proposition) {
            Ok(record) => record,
            Err(e) => {
                trace!(
                    "GraphicalModel::add_to_training_queue - Error serializing proposition: {}",
                    e
                );
                return Err(Box::new(e));
            }
        };
        trace!(
            "GraphicalModel::add_to_training_queue - Serialized proposition: {}",
            &serialized_proposition
        );
        if let Err(e) = self
            .redis_connection
            .borrow_mut()
            .rpush::<_, _, bool>(queue_name, &serialized_proposition)
        {
            trace!("GraphicalModel::add_to_training_queue - Error adding proposition to training queue in Redis: {}", e);
            return Err(Box::new(e));
        }
        trace!("GraphicalModel::add_to_training_queue - Proposition added to training queue successfully");
        Ok(())
    }

    pub fn maybe_add_to_training(
        &mut self,
        is_training: bool,
        proposition: &Proposition,
    ) -> Result<(), Box<dyn Error>> {
        if is_training {
            self.add_proposition_to_queue(&"training_queue".to_string(), &proposition)
        } else {
            Ok(())
        }
    }

    pub fn maybe_add_to_test(
        &mut self,
        is_test: bool,
        proposition: &Proposition,
    ) -> Result<(), Box<dyn Error>> {
        if is_test {
            self.add_proposition_to_queue(&"test_queue".to_string(), &proposition)
        } else {
            Ok(())
        }
    }

    fn get_propositions_from_queue(
        &self,
        seq_name: &String,
    ) -> Result<Vec<Proposition>, Box<dyn Error>> {
        trace!(
            "GraphicalModel::get_propositions_from_queue - Start. Queue name: {}",
            seq_name
        );
        let records = seq_get_all(&mut self.redis_connection.borrow_mut(), &seq_name)?;
        let mut result = vec![];
        for record in &records {
            let proposition = deserialize_record(record)?;
            result.push(proposition);
        }
        trace!("GraphicalModel::get_propositions_from_queue - Retrieved and deserialized propositions successfully");
        Ok(result)
    }

    pub fn get_training_questions(&self) -> Result<Vec<Proposition>, Box<dyn Error>> {
        let training_queue_name = String::from("training_queue");
        self.get_propositions_from_queue(&training_queue_name)
    }

    pub fn get_test_questions(&self) -> Result<Vec<Proposition>, Box<dyn Error>> {
        let test_queue_name = String::from("test_queue");
        self.get_propositions_from_queue(&test_queue_name)
    }
}

fn deserialize_record<'a, T>(record: &'a str) -> Result<T, Box<dyn Error>>
where
    T: Deserialize<'a>,
{
    serde_json::from_str(record).map_err(|e| Box::new(e) as Box<dyn Error>)
}

// Probabilities are either 0 or 1, so assume independent, i.e., just boolean combine them as AND.
fn extract_group_probability_for_training(
    proposition_db: &Box<dyn BeliefTable>,
    premise:&PropositionGroup,
) -> Result<f64, Box<dyn Error>> {
    let mut product = 1f64;
    for term in &premise.terms {
        let part = proposition_db.get_proposition_probability(term)?.unwrap();
        product *= part;
    }
    Ok(product)
}

fn extract_factor_for_proposition_for_training(
    proposition_db: &Box<dyn BeliefTable>,
    graph: &InferenceGraph,
    conclusion: Proposition,
) -> Result<FactorContext, Box<dyn Error>> {
    let factors = extract_backimplications_from_proposition(graph, &conclusion)?;
    let mut probabilities = vec![];
    for factor in &factors {
        let probability = extract_group_probability_for_training(proposition_db, &factor.premise)?;
        probabilities.push(probability);
    }
    let result = FactorContext {
        factor: factors,
        probabilities,
    };
    Ok(result)
}

pub fn do_training(resources: &FactoryResources) -> Result<(), Box<dyn Error>> {
    let graph = InferenceGraph::new_mutable(resources)?;
    let proposition_db = RedisBeliefTable::new_mutable(&resources.redis)?;
    let plan = TrainingPlan::new(&resources.redis)?;
    let mut factor_model = ExponentialModel::new_mutable(&resources)?;
    trace!("do_training - Getting all implications");
    let implications = graph.get_all_implications()?;
    for implication in implications {
        print_yellow!("do_training - Processing implication: {:?}", implication);
        factor_model.initialize_connection(&implication)?;
    }
    trace!("do_training - Getting all propositions");
    let training_questions = plan.get_training_questions()?;
    trace!(
        "do_training - Processing propositions: {}",
        training_questions.len()
    );
    let mut examples_processed = 0;
    for proposition in &training_questions {
        trace!("do_training - Processing proposition: {:?}", proposition);
        let factor = extract_factor_for_proposition_for_training(&proposition_db, &graph, proposition.clone())?;
        trace!("do_training - Backimplications: {:?}", &factor);
        let probabiity_opt = proposition_db.get_proposition_probability(proposition)?;
        let probability = probabiity_opt.expect("Probability should exist.");
        let _stats = factor_model.train(&factor, probability) ?;
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
    trace!("scenario result: {:?}", result);
    let train_result = do_training(resources);
    trace!("train result: {:?}", train_result);
    Ok(())
}
