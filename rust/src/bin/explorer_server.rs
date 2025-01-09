#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

use std::sync::{Arc, Mutex};

use bayes_star::{
    common::{
        graph::InferenceGraph,
        resources::ResourceContext,
        setup::{parse_configuration_options, CommandLineOptions},
    },
    explorer::{render_utils::{read_all_css, render_app_body}, routes::{experiment_route::internal_experiment, index_route::internal_index, network_route::internal_network}},
};
use redis::Connection;
use rocket::response::content::{Content, Html};
use rocket::{http::ContentType, State};
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

fn main() {
    let config = parse_configuration_options();
    rocket::ignite()
        .manage(AppContext::new(config))
        .mount("/", routes![home, experiment, network])
        .mount("/static", StaticFiles::from("static"))
        .launch();
}
