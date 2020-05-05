extern crate network_rocket;

use network_rocket::rocket_ins;
use rocket::http::Status;
use rocket::local::Client;

#[test]
fn route_user() {
    let client = Client::new(rocket_ins()).expect("valid rocket instance");
    let mut response = client.get("/user/123").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Hello, 123!".into()));
}

#[test]
fn route_int() {
    let client = Client::new(rocket_ins()).expect("valid rocket instance");
    let mut response = client.get("/user/-123").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Hello, -123!".into()));
}

#[test]
fn route_str() {
    let client = Client::new(rocket_ins()).expect("valid rocket instance");
    let mut response = client.get("/user/World").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Hello, World!".into()));
}
