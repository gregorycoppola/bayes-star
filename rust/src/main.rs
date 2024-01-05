use bayes_star::model::config::set_config;
use bayes_star::model::inference::inference_probability;
use bayes_star::model::{maxent::do_training, config::Config};
use bayes_star::model::storage::Storage;
use bayes_star::scenarios::dating1::{setup_train, setup_test};
use redis::Client;

#[macro_use]
extern crate log;
use clap::{App, Arg};

fn main() {
    env_logger::init();

    let matches = App::new("BAYES STAR")
        .version("1.0")
        .author("Greg Coppola")
        .about("Efficient combination of First-Order Logic and Bayesian Networks.")
        .arg(
            Arg::with_name("entities_per_domain")
                .long("entities_per_domain")
                .value_name("NUMBER")
                .help("Sets the number of entities per domain")
                .takes_value(true)
                .default_value("1024"),
        )
        .arg(
            Arg::with_name("print_training_loss")
                .long("print_training_loss")
                .help("Enables printing of training loss")
                .takes_value(false), // No value is expected, presence of flag sets it to true
        )
        .get_matches();

    let entities_per_domain: i32 = matches
        .value_of("entities_per_domain")
        .unwrap() // safe because we have a default value
        .parse()
        .expect("entities_per_domain needs to be an integer");

    let print_training_loss = matches.is_present("print_training_loss");
    set_config(Config {
        entities_per_domain,
        print_training_loss,
    }).expect("Could not set config.");

    // Create a Redis client
    let client = Client::open("redis://127.0.0.1/").expect("Could not connect to Redis."); // Replace with your Redis server URL

    let connection = client.get_connection().expect("Couldn't get connection.");
    // // Wrap the client in an Arc for shared ownership
    // let arc_client = Arc::new(client);

    // Create a new Storage instance
    let mut storage = Storage::new(connection).expect("Couldn't make storage");

    let result = setup_train(&mut storage);
    trace!("{:?}", result);

    let train_result = do_training(&mut storage);
    trace!("{:?}", train_result);

    let test_questions = setup_test(&mut storage).expect("Couldn't set up test.");


    for test_question in &test_questions {
        trace!("test_question {:?}", &test_question);

        let inference_result = inference_probability(&mut storage, &test_question);
        trace!("inference_result {:?}", &inference_result);
    }

    // Explicitly drop the Redis client
    std::mem::drop(storage);

    warn!("program done");
}
