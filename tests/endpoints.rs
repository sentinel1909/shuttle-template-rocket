// tests/httpd.rs

// integration test for the route at /
#[cfg(test)]
mod endpoint_tests {
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use shuttle_template_rocket::router::create;

    #[test]
    fn test_index() {
        let client = Client::tracked(create()).expect("valid rocket instance");
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }
}
