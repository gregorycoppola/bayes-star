use rocket::response::content::Html;

use crate::{common::resources::ResourceContext, explorer::render_utils::render_app_body, inference::rounds::run_inference_rounds};


pub fn internal_marginals(experiment_name: &str, test_scenario: &str, resource_context: &ResourceContext) -> Html<String> {
    let marginal_tables = run_inference_rounds(experiment_name, test_scenario, resource_context)
        .expect("Testing failed.");
    let result = render_app_body("");
    Html(result.unwrap())
}
