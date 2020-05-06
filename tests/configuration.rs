extern crate network_rocket;

use network_rocket::rocket_ins;
use rocket::http::Status;
use rocket::local::Client;

#[test]
fn test_config_in_assets_dir() {
    let client = Client::new(rocket_ins()).expect("valid rocket instance");
    let mut response = client.get("/config/dev.log").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.body_string(),
        Some("Route: /config/dev.log ---> Path: /assets/dev.log".into())
    );
}
