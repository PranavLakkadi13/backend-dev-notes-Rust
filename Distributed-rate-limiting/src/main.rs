use rocket::{get, http::Status, routes, launch};

use crate::rate_limiter::RateLimiter;

mod rate_limiter;

#[get("/health")]
fn health_handler() -> &'static str {
    "Hello I am UP....."
}

#[get("/429")]
fn custom_429_handler() -> Status {
    Status::TooManyRequests
}

#[launch]
fn rocket() -> _ {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();

    let middleware = RateLimiter { client };

    rocket::build()
        .attach(middleware)
        .mount("/", routes![health_handler, custom_429_handler])
}
