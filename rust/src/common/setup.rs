use crate::common::resources::ResourceContext;
use clap::{App, Arg};
use env_logger::{Builder, Env};
use serde::Deserialize;
use std::{io::Write, path::Path};

/// These options define the inputs from the user.
/// Nothing is owned by basic data types so this class can be easily freely around.
#[derive(Deserialize, Clone, Debug)]
pub struct CommandLineOptions {
    pub scenario_name: String,
    pub test_scenario: Option<String>,
    pub entities_per_domain: i32,
    pub print_training_loss: bool,
    pub test_example: Option<u32>,
    pub marginal_output_file: Option<String>,
}

fn check_file_does_not_exist(file_name: &str) {
    if Path::new(file_name).exists() {
        panic!("File '{}' already exists!", file_name);
    }
}

pub fn parse_configuration_options() -> CommandLineOptions {
    Builder::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| {
            let file = record.file().unwrap_or("unknown");
            let line = record.line().unwrap_or(0);
            writeln!(
                buf,
                "{} [{}:{}] {}",
                record.level(),
                file,
                line,
                record.args()
            )
        })
        .init();
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
        .arg(
            Arg::with_name("test_example")
                .long("test_example")
                .value_name("NUMBER")
                .help("Sets the test example number (optional)")
                .takes_value(true), // This argument is optional and takes a value
        )
        .arg(
            Arg::with_name("scenario_name")
                .long("scenario_name")
                .value_name("STRING")
                .help("Sets the scenario name")
                .takes_value(true)
                .required(true), // Mark this argument as required
        )
        .arg(
            Arg::with_name("test_scenario")
                .long("test_scenario")
                .value_name("STRING")
                .help("Test Scenario name")
                .takes_value(true)
                .required(false), // Mark this argument as required
        )
        .arg(
            Arg::with_name("marginal_output_file")
                .long("marginal_output_file")
                .value_name("FILE")
                .help("Sets the file name for marginal output (optional)")
                .takes_value(true), // This argument is optional and takes a string value
        )
        .get_matches();
    let entities_per_domain: i32 = matches
        .value_of("entities_per_domain")
        .unwrap() // safe because we have a default value
        .parse()
        .expect("entities_per_domain needs to be an integer");
    let print_training_loss = matches.is_present("print_training_loss");
    let test_example: Option<u32> = matches.value_of("test_example").map(|v| {
        v.parse()
            .expect("test_example needs to be a positive integer or omitted")
    });
    let marginal_output_file = matches.value_of("marginal_output_file").map(String::from);
    let scenario_name: String = matches
        .value_of("scenario_name")
        .expect("scenario_name is required") // As it's required, unwrap directly
        .to_string();
    let test_scenario = matches.value_of("test_scenario").map(String::from);

    CommandLineOptions {
        scenario_name,
        test_scenario,
        entities_per_domain,
        print_training_loss,
        test_example,
        marginal_output_file,
    }
}
