extern crate network_rocket;

use network_rocket::rocket_ins;
use rocket::http::Status;
use rocket::local::Client;

#[test]
fn not_found() {
    let client = Client::new(rocket_ins()).expect("valid rocket instance");
    let mut response = client.get("/404").dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(
        response.body_string(),
        Some("\'/404\' is not a valid path.".into())
    );
}
