extern crate network_rocket;

use network_rocket::rocket_ins;
use rocket::http::Header;
use rocket::local::Client;

#[test]
fn admin_user() {
    let client = Client::new(rocket_ins()).unwrap();
    let mut req = client.get("/admin");

    req.add_header(Header::new("user_role", "admin"));

    let mut response = req.dispatch();

    assert_eq!(
        Some("Hello, administrator. This is the admin panel!".into()),
        response.body_string()
    );
}

#[test]
fn user() {
    let client = Client::new(rocket_ins()).unwrap();
    let mut req = client.get("/admin");

    req.add_header(Header::new("user_role", "user"));

    let mut response = req.dispatch();

    assert_eq!(
        Some("Sorry, you must be an administrator to access this page.".into()),
        response.body_string()
    );
}

#[test]
fn anonymous_user() {
    let client = Client::new(rocket_ins()).unwrap();
    let req = client.get("/admin");
    let mut response = req.dispatch();

    // 服务将会重定向至 /login 页面，返回响应为空
    assert_eq!(None, response.body_string());
}
