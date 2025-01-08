use rocket::response::content::Html;

use crate::explorer::render_utils::render_app_body;


pub fn internal_index() -> Html<String> {
    let result = render_app_body("");
    Html(result.unwrap())
}
