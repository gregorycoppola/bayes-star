#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

use bayes_star::{
    common::{
        graph::InferenceGraph,
        resources::FactoryResources,
        setup::{parse_configuration_options, ConfigurationOptions},
    },
    explorer::{render::{read_all_css, render_app_body}, routes::{experiment_route::internal_experiment, index_route::internal_index}},
};
use rocket::response::content::{Content, Html};
use rocket::{http::ContentType, State};

struct AppContext {
    graph: InferenceGraph,
}

impl AppContext {
    pub fn new(config: ConfigurationOptions) -> Self {
        let resources = FactoryResources::new(&config).expect("Failed to create factory resources");
        let graph =
            InferenceGraph::new_literal(&resources).expect("Failed to create inference graph");
        AppContext { graph }
    }
}

#[get("/")]
fn home(_context: State<AppContext>) -> Html<String> {
    internal_index()
}

#[get("/experiment/<experiment_name>")]
fn experiment(experiment_name: String, _context: State<AppContext>) -> Html<String> {
    internal_experiment(&experiment_name)
}

fn main() {
    let config = parse_configuration_options();
    rocket::ignite()
        .manage(AppContext::new(config))
        .mount("/", routes![home, experiment])
        .launch();
}
