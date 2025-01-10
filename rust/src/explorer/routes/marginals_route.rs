use rocket::response::content::Html;

use crate::{common::resources::ResourceContext, explorer::render_utils::render_app_body};


pub fn internal_marginals(experiment_name: &str, test_scenario: &str, namespace: &ResourceContext) -> Html<String> {
    let result = render_app_body("");
    Html(result.unwrap())
}
