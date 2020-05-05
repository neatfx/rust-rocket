extern crate network_rocket;

use network_rocket::rocket_ins;
use rocket::http::Status;
use rocket::local::Client;

#[test]
fn path_with_one_dyn_segment() {
    let client = Client::new(rocket_ins()).expect("valid rocket instance");
    let mut response = client.get("/dyn/rustacean").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("rustacean".into()));
}

#[test]
fn path_with_multi_dyn_segments() {
    let client = Client::new(rocket_ins()).expect("valid rocket instance");
    let mut response = client.get("/dyn-n/rustacean/10/true").dispatch();
    let mut response_b = client.get("/dyn-n/rustacean/100/false").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("rustacean-10".into()));
    assert_eq!(response_b.status(), Status::Ok);
    assert_eq!(response_b.body_string(), Some("seg3 is false".into()));
}

#[test]
fn path_with_multiple_segments() {
    let client = Client::new(rocket_ins()).expect("valid rocket instance");
    let mut response = client.get("/file/foo.txt").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.body_string(),
        Some("file content in static/foo.txt".into())
    );
}
