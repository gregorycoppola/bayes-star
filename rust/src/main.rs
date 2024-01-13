use bayes_star::common::{run::setup_and_train, resources::FactoryResources};
use bayes_star::model::config::ConfigurationOptions;
use bayes_star::scenarios::dating_simple::SimpleDating;
use env_logger::{Builder, Env};
use std::io::Write;

#[macro_use]
extern crate log;
use clap::{App, Arg};

fn main() {
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
        .get_matches();
    let entities_per_domain: i32 = matches
        .value_of("entities_per_domain")
        .unwrap() // safe because we have a default value
        .parse()
        .expect("entities_per_domain needs to be an integer");
    let print_training_loss = matches.is_present("print_training_loss");
    // Run.
    let config = ConfigurationOptions {
        entities_per_domain,
        print_training_loss,
    };
    let resources = FactoryResources::new(&config).expect("Couldn't create resources.");
    let scenario_maker = SimpleDating {};
    setup_and_train(&resources, &scenario_maker).expect("Error in training.");
    warn!("program done");
}
