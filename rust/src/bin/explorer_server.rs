#[macro_use]
extern crate rocket;

use rocket::State;

struct RedisManager {
    pool: redis::Client,
}

impl RedisManager {
    pub async fn new(connection_string: &str) -> Self {
        let client =
            redis::Client::open(connection_string).expect("Redis client initialization failed");
        RedisManager { pool: client }
    }
}

#[get("/")]
fn index(redis: &State<RedisManager>) -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(RedisManager::new("redis://127.0.0.1/"))
        .mount("/", routes![index])
}
