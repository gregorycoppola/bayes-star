use rocket::response::content::Html;

use crate::explorer::render::render_app_body;


pub fn internal_experiment(experiment_name: &str) -> Html<String> {
    let result = render_app_body(experiment_name);
    Html(result.unwrap())
}