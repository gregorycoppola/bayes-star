use rocket::response::content::Html;

use crate::common::resources::ResourceContext;

pub fn internal_animation(
    experiment_name: &str,
    resource_context: &ResourceContext,
) -> Html<String> {
    // let graph = InferenceGraph::new_mutable(experiment_name.to_string()).unwrap();
    // let body_html = iterate_through_factors(experiment_name, resource_context).unwrap();
    // let result = render_app_body(&body_html);
    // Html(result.unwrap())
    todo!()
}
