// src/lib/router.rs

// dependencies
use crate::routes::index::index;
use rocket::routes;
use rocket::{Build, Rocket};

// function to create a Rocket instance
pub fn create() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}
