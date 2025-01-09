use crate::explorer::render_utils::render_app_body;


pub fn internal_index() -> String {
    let result = render_app_body("");
    result.unwrap()
}