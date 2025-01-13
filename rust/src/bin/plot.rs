use bayes_star::common::resources::ResourceContext;
use bayes_star::common::setup::parse_configuration_options;
use bayes_star::inference::rounds::run_inference_rounds;

extern crate log;

fn main() {
    let config = parse_configuration_options();
    let resources = ResourceContext::new(&config).expect("Couldn't create resources.");
    let test_scenario = config.test_scenario.expect("no test_scenario in config");
    let mut connection = resources.connection.lock().unwrap();
    let marginal_tables = run_inference_rounds(&mut connection, &config.scenario_name, &test_scenario)
        .expect("Testing failed.");
    for marginal_table in &marginal_tables {
        println!("table {:?}", marginal_table);
    }
    println!("main finishes");
}
