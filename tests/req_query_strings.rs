extern crate network_rocket;

use network_rocket::rocket_ins;
use rocket::http::Status;
use rocket::local::Client;

#[test]
fn query() {
    let client = Client::new(rocket_ins()).expect("valid rocket instance");
    let mut response = client.get("/query?name=John&wave").dispatch();
    let mut response_b = client.get("/query?wave&name=John").dispatch();
    let mut response_c = client.get("/query?wave&name=John&id=123").dispatch();
    let mut response_d = client.get("/query?id=123&name=John&wave").dispatch();
    let mut response_e = client.get("/query?name=Bob&name=John&wave").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("John".into()));

    assert_eq!(response_b.status(), Status::Ok);
    assert_eq!(response_b.body_string(), Some("John".into()));

    assert_eq!(response_c.status(), Status::Ok);
    assert_eq!(response_c.body_string(), Some("John".into()));

    assert_eq!(response_d.status(), Status::Ok);
    assert_eq!(response_d.body_string(), Some("John".into()));

    assert_eq!(response_e.status(), Status::Ok);
    assert_eq!(response_e.body_string(), Some("John".into()));
}

#[test]
fn query_with_optional_parameters() {
    let client = Client::new(rocket_ins()).expect("valid rocket instance");
    let mut response = client
        .get("/query_with_optional_para?name=John&wave&id=123")
        .dispatch();
    let mut response_b = client
        .get("/query_with_optional_para?id=123&wave")
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("John".into()));

    assert_eq!(response_b.status(), Status::Ok);
    assert_eq!(response_b.body_string(), Some("No name Parameter".into()));
}

#[test]
fn query_with_multiple_segements() {
    let client = Client::new(rocket_ins()).expect("valid rocket instance");
    let mut response = client
        .get("/query_with_multiple_segements?id=123&account=10000&name=Rustacean")
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.body_string(),
        Some("id - 123, user - Form(User { name: \"Rustacean\", account: 10000 })".into())
    );
}

#[test]
fn optional_item() {
    let client = Client::new(rocket_ins()).expect("valid rocket instance");
    let mut response = client.get("/optional-item?id=123&name=rust").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("id - 123, user - None".into()));
}
