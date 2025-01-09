use rocket::{get, routes, launch, Rocket};
use rocket::response::content::Content;
use rocket::http::ContentType;

// Simple HTML response
#[get("/")]
fn index() -> String {
    "<h1>Hello, world!</h1>".to_string()
}

// Explicit content type with Content wrapper
#[get("/welcome")]
fn welcome() -> Content<String> {
    Content(ContentType::HTML, "<h1>Welcome to the site!</h1>".to_string())
}

#[launch]
fn rocket() -> Rocket {
    rocket::build().mount("/", routes![index, welcome])
}
