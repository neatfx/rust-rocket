// Request Guards

use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::response::Redirect;
use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![login, admin_panel, admin_panel_user, admin_panel_redirect,]
}

#[derive(Debug)]
struct AdminUser(String);
#[derive(Debug)]
struct User(String);

impl<'a, 'r> FromRequest<'a, 'r> for AdminUser {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, !> {
        if request.headers().get_one("user_role") == Some("admin") {
            return Outcome::Success(AdminUser("admin".into()));
        }

        rocket::Outcome::Forward(())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, !> {
        if request.headers().get_one("user_role") == Some("user") {
            return Outcome::Success(User("user".into()));
        }

        rocket::Outcome::Forward(())
    }
}

#[get("/login")]
fn login() -> &'static str {
    "Please login!"
}

#[get("/admin")]
fn admin_panel(admin: AdminUser) -> &'static str {
    println!("admin_panel - {:#?}", admin);
    "Hello, administrator. This is the admin panel!"
}

#[get("/admin", rank = 2)]
fn admin_panel_user(user: User) -> &'static str {
    println!("admin_panel_user - {:#?}", user);
    "Sorry, you must be an administrator to access this page."
}

#[get("/admin", rank = 3)]
fn admin_panel_redirect() -> Redirect {
    Redirect::to("/login")
}
