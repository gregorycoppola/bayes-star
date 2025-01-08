use rocket::response::content::Html;

use crate::{common::graph::InferenceGraph, explorer::render_utils::render_app_body};


fn render_network(graph: &InferenceGraph) -> String {
    "".to_string()
}

pub fn internal_network(graph: &InferenceGraph) -> Html<String> {
    let network = render_network(graph);
    let body_html = format!(
        r#"
        {network}
    "#,
    );
    let result = render_app_body(&body_html);
    Html(result.unwrap())
}
