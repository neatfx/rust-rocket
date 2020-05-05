// Forwarding

use rocket::http::RawStr;
use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![user, user_int, user_str,]
}

#[get("/user/<id>")]
fn user(id: usize) -> String {
    format!("Hello, {}!", id)
}

#[get("/user/<id>", rank = 2)]
fn user_int(id: isize) -> String {
    format!("Hello, {}!", id)
}

#[get("/user/<id>", rank = 3)]
fn user_str(id: &RawStr) -> String {
    format!("Hello, {}!", id)
}
