// src/lib/routes/index.rs

// dependencies
use rocket::get;

// index route handler
#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}
