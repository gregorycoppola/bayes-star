use rocket::launch;
use rocket::{get, routes, Rocket, Build};
use rocket::{response::{self, Response}, http::ContentType};
use std::io::Cursor;
// // Dynamically created HTML response
// #[get("/")]
// fn index() -> String {
//     let greeting = "Hello, world!";
//     // format!("<h1>{}</h1>", greeting)
//     format!("<!DOCTYPE html><html><head><title>Home Page</title></head><body><h1>{}</h1></body></html>", greeting)
// }

#[get("/")]
fn index() -> response::Result<'static> {
    let greeting = "Hello, world!";
    let html_content = format!("<!DOCTYPE html><html><head><title>Home Page</title></head><body><h1>{}</h1></body></html>", greeting);
    Response::build()
        .header(ContentType::HTML)
        .sized_body(None, Cursor::new(html_content))
        .ok()
}

// Another example of dynamically created HTML
#[get("/welcome")]
fn welcome() -> String {
    let page_title = "Welcome to the site!";
    format!("<h1>{}</h1>", page_title)
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, welcome])
}
