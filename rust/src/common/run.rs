use std::cell::RefCell;
use std::error::Error;

use redis::{Client, Connection};

use super::interface::{FactorModel, ScenarioMaker};
use crate::model::maxent::ExponentialModel;
use crate::model::{inference::marginalized_inference_probability, storage::Graph};

fn create_model(
    model_name: &String,
    connection: RefCell<Connection>,
) -> Result<Box<dyn FactorModel>, Box<dyn Error>> {
    Ok(Box::new(ExponentialModel::new(connection)))
}

pub fn do_training(
    model: &mut Box<dyn FactorModel>,
    storage: &Graph,
) -> Result<(), Box<dyn Error>> {
    trace!("do_training - Getting all implications");
    let implications = storage.get_all_implications()?;
    for implication in implications {
        trace!("do_training - Processing implication: {:?}", implication);
        model.initialize_connection(&implication)?;
    }

    trace!("do_training - Getting all propositions");
    let propositions = storage.get_training_questions()?;
    trace!(
        "do_training - Processing propositions: {}",
        propositions.len()
    );

    let mut examples_processed = 0;
    for proposition in propositions {
        match model.train(storage, &proposition) {
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

fn run_test_loop(storage: &mut Graph) -> Result<(), Box<dyn Error>> {
    let test_questions = storage
        .get_test_questions()
        .expect("Couldn't get test questions.");
    for test_question in &test_questions {
        println!("\n\n\n\n\n\n");
        info!("test_question {:?}", &test_question.search_string());
        let inference_result = marginalized_inference_probability(storage, &test_question);
        trace!("inference_result {:?}", &inference_result);
    }

    Ok(())
}

pub fn train_and_test(scenario_maker: &dyn ScenarioMaker) -> Result<(), Box<dyn Error>> {
    let client = Client::open("redis://127.0.0.1/").expect("Could not connect to Redis."); // Replace with your Redis server URL
    let connection = client.get_connection().expect("Couldn't get connection.");
    let mut storage = Graph::new(connection).expect("Couldn't make storage");
    let result = scenario_maker.setup_scenario(&mut storage);
    info!("scenario result: {:?}", result);

    let mut model = create_model(
        &"model_name".to_string(),
        RefCell::new(client.get_connection().expect("Couldn't get connection.")),
    )?;
    let train_result = do_training(&mut model, &storage);
    info!("train result: {:?}", train_result);

    run_test_loop(&mut storage)?;
    // Explicitly drop the Redis client
    std::mem::drop(storage);

    Ok(())
}