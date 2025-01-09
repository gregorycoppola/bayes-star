use crate::{
    common::{
        interface::BeliefTable,
        redis::{seq_get_all, seq_push},
    },
    model::{
        self,
        exponential::ExponentialModel,
        objects::{
            Domain, Entity, Predicate, ImplicationFactor, PredicateGroup, Proposition,
            PropositionGroup,
        },
    },
    print_yellow,
};
use redis::{Commands, Connection};
use serde::Deserialize;
use std::{cell::RefCell, error::Error, sync::{Arc, Mutex}};

use super::graph::InferenceGraph;
use super::interface::ScenarioMaker;
use super::model::FactorModel;
use super::resources::ResourceContext;
use super::{
    interface::{PredictStatistics, TrainStatistics},
    model::FactorContext,
    redis::RedisManager,
};
use crate::common::model::InferenceModel;
use crate::common::proposition_db::RedisBeliefTable;
use crate::model::choose::extract_backimplications_from_proposition;
use std::borrow::BorrowMut;

pub struct TrainingPlan {
    namespace: String,
}

impl TrainingPlan {
    pub fn new(namespace: String) -> Result<Self, Box<dyn Error>> {
        Ok(TrainingPlan {
            namespace,
        })
    }

    pub fn add_proposition_to_queue(
        &mut self,
        connection: &mut Connection,
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
        seq_push(
             connection,
            &self.namespace,
            &queue_name,
            &serialized_proposition,
        )?;
        trace!("GraphicalModel::add_to_training_queue - Proposition added to training queue successfully");
        Ok(())
    }

    pub fn maybe_add_to_training(
        &mut self,
        connection: &mut Connection,
        is_training: bool,
        proposition: &Proposition,
    ) -> Result<(), Box<dyn Error>> {
        if is_training {
            self.add_proposition_to_queue(connection, &"training_queue".to_string(), &proposition)
        } else {
            Ok(())
        }
    }

    pub fn maybe_add_to_test(
        &mut self,
        connection: &mut Connection,
        is_test: bool,
        proposition: &Proposition,
    ) -> Result<(), Box<dyn Error>> {
        if is_test {
            self.add_proposition_to_queue(connection, &"test_queue".to_string(), &proposition)
        } else {
            Ok(())
        }
    }

    fn get_propositions_from_queue(
        &self,
        connection: &mut Connection,
        seq_name: &String,
    ) -> Result<Vec<Proposition>, Box<dyn Error>> {
        trace!(
            "GraphicalModel::get_propositions_from_queue - Start. Queue name: {}",
            seq_name
        );
        let records = seq_get_all(
            connection,
            &self.namespace,
            &seq_name,
        )?;
        let mut result = vec![];
        for record in &records {
            let proposition = deserialize_record(record)?;
            result.push(proposition);
        }
        trace!("GraphicalModel::get_propositions_from_queue - Retrieved and deserialized propositions successfully");
        Ok(result)
    }

    pub fn get_training_questions(&self, connection: &mut Connection) -> Result<Vec<Proposition>, Box<dyn Error>> {
        let training_queue_name = String::from("training_queue");
        self.get_propositions_from_queue(connection, &training_queue_name)
    }

    pub fn get_test_questions(&self, connection: &mut Connection) -> Result<Vec<Proposition>, Box<dyn Error>> {
        let test_queue_name = String::from("test_queue");
        self.get_propositions_from_queue(connection, &test_queue_name)
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
    connection: &mut Connection,
    proposition_db: &Box<dyn BeliefTable>,
    premise: &PropositionGroup,
) -> Result<f64, Box<dyn Error>> {
    let mut product = 1f64;
    for term in &premise.terms {
        let part = proposition_db.get_proposition_probability(connection, term)?.unwrap();
        product *= part;
    }
    Ok(product)
}

fn extract_factor_for_proposition_for_training(
    connection: &mut Connection,
    proposition_db: &Box<dyn BeliefTable>,
    graph: &InferenceGraph,
    conclusion: Proposition,
) -> Result<FactorContext, Box<dyn Error>> {
    let factors = extract_backimplications_from_proposition(connection, graph, &conclusion)?;
    let mut probabilities = vec![];
    for factor in &factors {
        let probability = extract_group_probability_for_training(connection, proposition_db, &factor.premise)?;
        probabilities.push(probability);
    }
    let result = FactorContext {
        factor: factors,
        probabilities,
    };
    Ok(result)
}

pub fn do_training(resources: &ResourceContext, namespace: String) -> Result<(), Box<dyn Error>> {
    let mut connection = resources.connection.lock().unwrap();
    let graph = InferenceGraph::new_mutable(namespace.clone())?;
    let proposition_db = RedisBeliefTable::new_mutable(namespace.clone())?;
    let plan = TrainingPlan::new(namespace.clone())?;
    let mut factor_model = ExponentialModel::new_mutable(namespace.clone())?;
    trace!("do_training - Getting all implications");
    let implications = graph.get_all_implications(&mut connection)?;
    for implication in implications {
        print_yellow!("do_training - Processing implication: {:?}", implication);
        factor_model.initialize_connection(&implication)?;
    }
    trace!("do_training - Getting all propositions");
    let training_questions = plan.get_training_questions(&mut connection)?;
    trace!(
        "do_training - Processing propositions: {}",
        training_questions.len()
    );
    let mut examples_processed = 0;
    for proposition in &training_questions {
        trace!("do_training - Processing proposition: {:?}", proposition);
        let factor = extract_factor_for_proposition_for_training(
            &mut connection,
            &proposition_db,
            &graph,
            proposition.clone(),
        )?;
        trace!("do_training - Backimplications: {:?}", &factor);
        let probabiity_opt = proposition_db.get_proposition_probability(&mut connection, proposition)?;
        let probability = probabiity_opt.expect("Probability should exist.");
        let _stats = factor_model.train(&factor, probability)?;
        examples_processed += 1;
    }
    trace!(
        "do_training - Training complete: examples processed {}",
        examples_processed
    );
    Ok(())
}

pub fn setup_and_train(
    resources: &ResourceContext,
    scenario_maker: &dyn ScenarioMaker,
    namespace: &str,
) -> Result<(), Box<dyn Error>> {
    let model_spec = "dummy_model_spec".to_string();
    let result = scenario_maker.setup_scenario(resources);
    trace!("scenario result: {:?}", result);
    let train_result = do_training(resources,  namespace.to_string());
    trace!("train result: {:?}", train_result);
    Ok(())
}
