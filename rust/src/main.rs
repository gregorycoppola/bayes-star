use bayes_star::model::maxent::do_training;
use bayes_star::model::storage::Storage;
use bayes_star::scenarios::dating1::setup_scenario;
use redis::Client;
use std::sync::Arc; // Replace `your_crate` with the name of your crate
use env_logger::Env;
#[macro_use]
extern crate log;

fn main() {
    env_logger::init();
        // Create a Redis client
    let client = Client::open("redis://127.0.0.1/").expect("Could not connect to Redis."); // Replace with your Redis server URL

    let connection = client.get_connection().expect("Couldn't get connection.");
    // // Wrap the client in an Arc for shared ownership
    // let arc_client = Arc::new(client);

    // Create a new Storage instance
    let mut storage = Storage::new(connection).expect("Couldn't make storage");

    let result = setup_scenario(&mut storage);
    trace!("{:?}", result);

    let train_result = do_training(&mut storage);
    trace!("{:?}", train_result);

    // Explicitly drop the Redis client
    std::mem::drop(storage);
}
