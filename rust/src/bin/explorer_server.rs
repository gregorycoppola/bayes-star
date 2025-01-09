#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use rocket::{State, Rocket};
use rocket::response::content::Html;
use r2d2_redis::{r2d2, RedisConnectionManager};
use rocket_contrib::serve::StaticFiles;

pub struct AppContext {
    // Redis pool will now be part of AppContext if needed, or it can be accessed directly from the state in handlers
    // graph: InferenceGraph, (Assumed existing)
    // config: CommandLineOptions, (Assumed existing)
}

impl AppContext {
    pub fn new() -> Self {
        // Initialization of other parts of the context if needed
        AppContext {
            // graph and config initialization
        }
    }
}

#[get("/")]
fn home() -> Html<String> {
    // Your existing home handling logic
    Html("<h1>Welcome Home</h1>".to_string())
}

#[get("/experiment/<experiment_name>")]
fn experiment(experiment_name: String, redis: &State<r2d2::Pool<RedisConnectionManager>>) -> Html<String> {
    let mut conn = redis.get().expect("Failed to get Redis connection");
    // Use Redis connection for your experiment logic
    Html(format!("<h1>Experiment: {}</h1>", experiment_name))
}

#[get("/network/<experiment_name>")]
fn network(experiment_name: String, redis: &State<r2d2::Pool<RedisConnectionManager>>) -> Html<String> {
    let mut conn = redis.get().expect("Failed to get Redis connection");
    // Use Redis connection for your network logic
    Html(format!("<h1>Network for Experiment: {}</h1>", experiment_name))
}

fn rocket() -> Rocket {
    let redis_url = "redis://127.0.0.1/";
    let manager = RedisConnectionManager::new(redis_url).expect("Invalid Redis URL");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create Redis pool");

    rocket::ignite()
        .manage(AppContext::new())
        .manage(pool) // Manage the Redis pool directly
        .mount("/", routes![home, experiment, network])
        .mount("/static", StaticFiles::from("static"))
}

fn main() {
    rocket().launch();
}
