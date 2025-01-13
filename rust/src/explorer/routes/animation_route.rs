use rocket::response::content::Html;

use crate::{common::resources::ResourceContext, explorer::render_utils::render_app_body, inference::rounds::run_inference_rounds};

fn internal_internal_marginals(experiment_name: &str, test_scenario: &str, resource_context: &ResourceContext) -> String {
    todo!()
}

pub fn internal_animation(
    experiment_name: &str,
    test_scenario: &str,
    resource_context: &ResourceContext,
) -> Html<String> {
    let marginal_tables = run_inference_rounds(experiment_name, test_scenario, resource_context)
        .expect("Testing failed.");

    let mut body_html = "".to_string();
    body_html += &format!("<div class='marginal_box'>");
    for marginal_table in &marginal_tables {
        let html_part = marginal_table.render_marginal_table();
        body_html += &html_part;
    }
    body_html += &format!("</div>");
    let result = render_app_body(&body_html);
    Html(result.unwrap())
}
