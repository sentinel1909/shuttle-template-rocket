// src/lib/router.rs

// dependencies
use rocket::{Build, Rocket};
use rocket::routes;
use crate::routes::index::index;

// function to create a Rocket instance
pub fn create() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}
