#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

use bayes_star::{
    common::{
        graph::InferenceGraph,
        resources::FactoryResources,
        setup::{parse_configuration_options, ConfigurationOptions},
    },
    explorer::{render::{read_all_css, render_app_body}, routes::index_route::internal_index},
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
fn home() -> Html<String> {
    internal_index()
}

fn main() {
    let config = parse_configuration_options();
    rocket::ignite()
        .manage(AppContext::new(config))
        .mount("/", routes![home])
        .launch();
    // .mount("/", routes![home, domains, relations, implications])
}
