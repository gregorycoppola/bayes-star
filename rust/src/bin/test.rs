use bayes_star::common::setup::parse_configuration_options;
use bayes_star::common::resources::FactoryResources;
use bayes_star::common::test::{interactive_inference_example, summarize_examples};

extern crate log;

fn main() {
    let config: bayes_star::common::setup::CommandLineOptions = parse_configuration_options();
    let resources = FactoryResources::new(&config).expect("Couldn't create resources.");
    let test_example = config.test_example;
    if test_example.is_some() {
        interactive_inference_example(&config, &resources).expect("Testing failed.");
    } else {
        summarize_examples(&config, &resources).expect("Summarize failed.");
    }
    println!("main finishes");
}
