#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

use bayes_star::{
    common::{
        resources::ResourceContext,
        setup::{parse_configuration_options, CommandLineOptions},
    },
    explorer::routes::{experiment_route::internal_experiment, index_route::internal_index, network_route::internal_network, weights_route::internal_weights},
};
use rocket::response::content::Html;
use rocket::State;
use rocket_contrib::serve::StaticFiles;

pub struct AppContext {
    namespace: ResourceContext,
}

impl AppContext {
    pub fn new(config: CommandLineOptions) -> Self {
        let namespace = ResourceContext::new(&config).expect("Failed to create factory resources");
        AppContext { namespace }
    }
}

#[get("/")]
fn home(_context: State<AppContext>) -> Html<String> {
    internal_index()
}

#[get("/experiment/<experiment_name>")]
fn experiment(experiment_name: String, context: State<AppContext>) -> Html<String> {
    internal_experiment(&experiment_name, &context.namespace)
}

#[get("/network/<experiment_name>")]
fn network(experiment_name: String, context: State<AppContext>) -> Html<String> {
    internal_network(&experiment_name, &context.namespace)
}

#[get("/weights/<experiment_name>")]
fn weights(experiment_name: String, context: State<AppContext>) -> Html<String> {
    internal_weights(&experiment_name, &context.namespace)
}

fn main() {
    let config = parse_configuration_options();
    rocket::ignite()
        .manage(AppContext::new(config))
        .mount("/", routes![home, experiment, network, weights])
        .mount("/static", StaticFiles::from("static"))
        .launch();
}
