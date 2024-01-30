use crate::common::resources::FactoryResources;
use crate::scenarios::dating_simple::SimpleDating;
use clap::{App, Arg};
use env_logger::{Builder, Env};
use serde::Deserialize;
use std::io::Write;

#[derive(Deserialize, Clone, Debug)]
pub struct ConfigurationOptions {
    pub scenario_name: String,
    pub entities_per_domain: i32,
    pub print_training_loss: bool,
    pub test_example: Option<u32>,
}

pub fn parse_configuration_options() -> ConfigurationOptions {
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

    let scenario_name: String = matches
        .value_of("scenario_name")
        .expect("scenario_name is required") // As it's required, unwrap directly
        .to_string();

    ConfigurationOptions {
        scenario_name,
        entities_per_domain,
        print_training_loss,
        test_example,
    }
}
