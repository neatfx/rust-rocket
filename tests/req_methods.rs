extern crate network_rocket;

use network_rocket::rocket_ins;
use rocket::http::Status;
use rocket::local::Client;

#[test]
fn index() {
    let client = Client::new(rocket_ins()).expect("valid rocket instance");
    let mut response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Hello, world!".into()));
}

#[test]
fn get() {
    let client = Client::new(rocket_ins()).expect("valid rocket instance");
    let mut response = client.get("/a").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("route a".into()));
}

#[test]
fn route_with_namespaced_path() {
    let client = Client::new(rocket_ins()).expect("valid rocket instance");
    let mut response = client.get("/b").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("route b".into()));
}
