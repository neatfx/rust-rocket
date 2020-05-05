use rocket::Catcher;

mod customized_error_catchers;
mod standard_http_error_catchers;

pub fn catchers() -> Vec<Catcher> {
    catchers![standard_http_error_catchers::not_found]
}
