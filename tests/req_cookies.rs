extern crate network_rocket;

use network_rocket::rocket_ins;
use rocket::http::{ContentType, Cookie, Status};
use rocket::local::Client;

#[test]
fn get_cookie() {
    let client = Client::new(rocket_ins()).unwrap();
    let response = client.get("/cookies").dispatch();

    let cookie_headers: Vec<_> = response.headers().get("Set-Cookie").collect();
    let location_headers: Vec<_> = response.headers().get("Location").collect();

    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(
        cookie_headers,
        vec!["message=cookie%20from%20server".to_string()]
    );
    assert_eq!(location_headers, vec!["/cookies/get".to_string()]);
}

#[test]
fn set_cookie_with_client_data() {
    let client = Client::new(rocket_ins()).unwrap();
    let response = client
        .post("/cookies")
        .header(ContentType::Form)
        .body("message=post data from client")
        .dispatch();

    let cookie_headers: Vec<_> = response.headers().get("Set-Cookie").collect();
    let location_headers: Vec<_> = response.headers().get("Location").collect();

    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(
        cookie_headers,
        vec!["message=post%20data%20from%20client".to_string()]
    );
    assert_eq!(location_headers, vec!["/cookies/get".to_string()]);
}

#[test]
fn request_with_cookie() {
    let client = Client::new(rocket_ins()).unwrap();
    let mut response = client
        .get("/cookies/get")
        .cookie(Cookie::new("message", "Cookie from Rocket!"))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Cookie from Rocket!".into()));
}

#[test]
fn request_without_cookie() {
    let client = Client::new(rocket_ins()).unwrap();
    let mut response = client.get("/cookies/get").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("No cookie from client".into()));
}

#[test]
fn get_private_cookie() {
    let client = Client::new(rocket_ins()).unwrap();
    let mut response = client.get("/cookies/private").dispatch();

    let cookie_headers: Vec<_> = response.headers().get("Set-Cookie").collect();

    assert_eq!(response.status(), Status::Ok);
    assert!(cookie_headers[0].contains("Path=/;"));
    assert_eq!(
        response.body_string(),
        Some("Cookie sended to client".into())
    );
}

#[test]
fn send_private_cookie() {
    // rustacean => TiiKwxJsf+e73qc0K9jXd+OMznbZn886JiA15WXvunACIe+ssg==
    let client = Client::new(rocket_ins()).unwrap();
    let mut response = client
        .get("/cookies/private/parse")
        .cookie(Cookie::new(
            "user_id_p",
            "TiiKwxJsf+e73qc0K9jXd+OMznbZn886JiA15WXvunACIe+ssg==",
        ))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("rustacean".into()));
}

#[test]
fn remove_private_cookie() {
    let client = Client::new(rocket_ins()).unwrap();
    let mut response = client.post("/cookies/private/remove").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.body_string(),
        Some("Successfully removed private cookie.".into())
    );
}
