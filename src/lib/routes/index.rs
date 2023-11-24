// src/lib/routes/index.rs

// dependencies
use rocket::get;

// index route handler
#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

// unit test for the route at /
#[cfg(test)]
mod endpoint_tests {
    use crate::router::create;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_index() {
        let client = Client::tracked(create()).expect("valid rocket instance");
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }
}
