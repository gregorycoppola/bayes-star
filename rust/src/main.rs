use bayes_star::model::storage::Storage;
use bayes_star::scenarios::dating1::SetupScenario;
use redis::Client;
use std::sync::Arc; // Replace `your_crate` with the name of your crate

fn main() {
    // Create a Redis client
    let client = Client::open("redis://127.0.0.1/").expect("Could not connect to Redis."); // Replace with your Redis server URL

    // Wrap the client in an Arc for shared ownership
    let arc_client = Arc::new(client);

    // Create a new Storage instance
    let storage = Storage::new(arc_client);

    // Now you can use your storage instance...
    // For example, getting the client back:
    let redis_client = storage.get_redis_client();
    println!("Redis client created and stored in Storage instance");

    let result = SetupScenario();
    println!("{:?}", result);
}
