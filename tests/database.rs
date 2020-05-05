extern crate network_rocket;

use network_rocket::rocket_ins;
use rocket::http::Status;
use rocket::local::Client;

#[test]
fn test_db_redis() {
    let client = Client::new(rocket_ins()).expect("valid rocket instance");
    let mut response = client.get("/database").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("42".into()));
}
