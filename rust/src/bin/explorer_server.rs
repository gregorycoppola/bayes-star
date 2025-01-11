#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

use bayes_star::{
    common::{
        resources::ResourceContext,
        setup::{parse_configuration_options, CommandLineOptions},
    },
    explorer::routes::{animation_route::internal_animation, experiment_route::internal_experiment, factors_route::internal_factors, index_route::internal_index, marginals_route::internal_marginals, network_route::internal_network, weights_route::internal_weights},
};
use rocket::response::content::Html;
use rocket::State;
use rocket_contrib::serve::StaticFiles;

pub struct WebContext {
    namespace: ResourceContext,
}

impl WebContext {
    pub fn new(config: CommandLineOptions) -> Self {
        let namespace = ResourceContext::new(&config).expect("Failed to create factory resources");
        WebContext { namespace }
    }
}

#[get("/")]
fn home(_context: State<WebContext>) -> Html<String> {
    internal_index()
}

#[get("/experiment/<experiment_name>")]
fn experiment(experiment_name: String, context: State<WebContext>) -> Html<String> {
    internal_experiment(&experiment_name, &context.namespace)
}

#[get("/network/<experiment_name>")]
fn network(experiment_name: String, context: State<WebContext>) -> Html<String> {
    internal_network(&experiment_name, &context.namespace)
}

#[get("/weights/<experiment_name>")]
fn weights(experiment_name: String, context: State<WebContext>) -> Html<String> {
    internal_weights(&experiment_name, &context.namespace)
}

#[get("/marginals/<experiment_name>/<test_scenario>")]
fn marginals(experiment_name: String, test_scenario: String, context: State<WebContext>) -> Html<String> {
    internal_marginals(&experiment_name, &test_scenario, &context.namespace)
}

#[get("/factors/<experiment_name>")]
fn factors(experiment_name: String, context: State<WebContext>) -> Html<String> {
    internal_factors(&experiment_name, &context.namespace)
}

#[get("/animation/<experiment_name>")]
fn animation(experiment_name: String, context: State<WebContext>) -> Html<String> {
    internal_animation(&experiment_name, &context.namespace)
}

fn main() {
    let config = parse_configuration_options();
    rocket::ignite()
        .manage(WebContext::new(config))
        .mount("/", routes![home, experiment, network, weights, marginals, factors, animation])
        .mount("/static", StaticFiles::from("static"))
        .launch();
}
