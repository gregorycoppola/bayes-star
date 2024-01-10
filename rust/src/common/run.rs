use std::error::Error;

use redis::Client;

use crate::model::{storage::Storage, maxent::do_training, inference::marginalized_inference_probability};

use super::interface::ScenarioMaker;

pub fn train_and_test(scenario_maker:&dyn ScenarioMaker) -> Result<(), Box<dyn Error>> {
    let client = Client::open("redis://127.0.0.1/").expect("Could not connect to Redis."); // Replace with your Redis server URL
    let connection = client.get_connection().expect("Couldn't get connection.");
    let mut storage = Storage::new(connection).expect("Couldn't make storage");
    let result = scenario_maker.setup_scenario(&mut storage);
    trace!("{:?}", result);

    let train_result = do_training(&mut storage);
    trace!("{:?}", train_result);

    let test_questions = storage.get_test_questions().expect("Couldn't get test questions.");
    for test_question in &test_questions {
        println!("\n\n\n\n\n\n");
        info!("test_question {:?}", &test_question.search_string());

        let inference_result = marginalized_inference_probability(&mut storage, &test_question);
        trace!("inference_result {:?}", &inference_result);
    }

    // Explicitly drop the Redis client
    std::mem::drop(storage);
    
    Ok(())
}