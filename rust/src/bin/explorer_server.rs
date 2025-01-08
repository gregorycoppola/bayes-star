#[macro_use]
extern crate rocket;

use rocket::{http::ContentType, State};
use bayes_star::{common::{graph::InferenceGraph, resources::FactoryResources, setup::{parse_configuration_options, ConfigurationOptions}}, explorer::render::render_app_body};
use rocket::response::content::{Content, Html};

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

// #[get("/")]
// fn home(_context: &State<AppContext>) -> Content<String> {
//     let result = render_app_body("");
//     Content(ContentType::HTML, result.unwrap())
// }

fn main() {
    let config = parse_configuration_options();
    rocket::build()
        .manage(AppContext::new(config))
        .mount("/", routes![])
        // .mount("/", routes![home, domains, relations, implications])
}
