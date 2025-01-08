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

#[get("/")]
fn home(_context: &State<AppContext>) -> Content<String> {
    let result = render_app_body("");
    Content(ContentType::HTML, result.unwrap())
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
        .mount("/", routes![home, domains, relations, implications])
}
