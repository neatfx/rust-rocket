extern crate network_rocket;

use network_rocket::rocket_ins;
use rocket::http::Status;
use rocket::http::{Accept, ContentType};
use rocket::local::Client;

#[test]
fn post_with_invalid_format() {
    let client = Client::new(rocket_ins()).expect("valid rocket instance");
    let response = client
        .post("/format")
        .header(ContentType::FormData)
        .body(r#"{ "content": "Hello, world!", "id": 90000 }"#)
        .dispatch();

    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn post_json_format() {
    let client = Client::new(rocket_ins()).expect("valid rocket instance");
    let mut response = client
        .post("/format")
        .header(ContentType::JSON)
        .body(r#"{ "content": "Hello, world!", "id": 60000 }"#)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("60000".into()));
}

#[test]
fn get_with_invalid_format() {
    let client = Client::new(rocket_ins()).expect("valid rocket instance");
    let response = client.get("/format/1").header(Accept::HTML).dispatch();

    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn get_json_format() {
    let client = Client::new(rocket_ins()).expect("valid rocket instance");
    let mut response = client.get("/format/1").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("{\"content\":\"message from rust\",\"id\":1}".into()));
}
