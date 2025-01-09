use rocket::launch;
use rocket::{get, routes, Rocket, Build};

// Dynamically created HTML response
#[get("/")]
fn index() -> String {
    let greeting = "Hello, world!";
    format!("<h1>{}</h1>", greeting)
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
