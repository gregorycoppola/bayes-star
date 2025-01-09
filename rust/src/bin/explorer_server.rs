#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

use bayes_star::{
    common::{
        graph::InferenceGraph,
        resources::FactoryResources,
        setup::{parse_configuration_options, CommandLineOptions},
    },
    explorer::routes::{experiment_route::internal_experiment, index_route::internal_index, network_route::internal_network},
};
use rocket::response::content::Html;
use rocket::State;
use rocket_contrib::serve::StaticFiles;

pub struct AppContext {
    graph: InferenceGraph,
    config: CommandLineOptions,
}

impl AppContext {
    pub fn new(config: CommandLineOptions) -> Self {
        let resources = FactoryResources::new(&config).expect("Failed to create factory resources");
        let graph =
            InferenceGraph::new_literal(&resources).expect("Failed to create inference graph");
        AppContext { graph, config }
    }
}

#[get("/")]
fn home(_context: State<AppContext>) -> Html<String> {
    internal_index()
}

#[get("/experiment/<experiment_name>")]
fn experiment(experiment_name: String, context: State<AppContext>) -> Html<String> {
    internal_experiment(&experiment_name, &context.graph)
}

#[get("/network/<experiment_name>")]
fn network(experiment_name: String, context: State<AppContext>) -> Html<String> {
    internal_network(&experiment_name, &context.graph, &context.config)
}

fn main() {
    let config = parse_configuration_options();
    rocket::ignite()
        .manage(AppContext::new(config))
        .mount("/", routes![home, experiment, network])
        .mount("/static", StaticFiles::from("static"))
        .launch();
}
