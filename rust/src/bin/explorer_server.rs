#[macro_use]
extern crate rocket;

use rocket::State;
use bayes_star::common::{graph::InferenceGraph, resources::FactoryResources, setup::{parse_configuration_options, ConfigurationOptions}};

struct AppContext {
    graph: InferenceGraph,
}

impl AppContext {
    pub fn new(config: ConfigurationOptions) -> Self {
        let resources = FactoryResources::new(&config).expect("Failed to create factory resources");
        let graph = InferenceGraph::new_literal(&resources).expect("Failed to create inference graph");
        AppContext { graph }
    }
}

#[get("/domains")]
fn domains(context: &State<AppContext>) -> String {
    let all_domains = context.graph.get_all_domains().expect("Failed to get domains");
    format!("{:?}", all_domains)
}

#[get("/relations")]
fn relations(context: &State<AppContext>) -> String {
    let all_relations = context.graph.get_all_relations().expect("Failed to get relations");
    format!("{:?}", all_relations)
}

#[get("/implications")]
fn implications(context: &State<AppContext>) -> String {
    let all_implications = context.graph.get_all_implications().expect("Failed to get implications");
    format!("{:?}", all_implications)
}

#[launch]
fn rocket() -> _ {
    let config = parse_configuration_options();
    rocket::build()
        .manage(AppContext::new(config))
        .mount("/", routes![domains, relations, implications])
}
