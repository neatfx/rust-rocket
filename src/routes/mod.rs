use rocket::Route;

mod configuration;
mod cookies;
mod database;
mod dynamic_paths;
mod format;
mod forwarding;
mod methods;
mod query_strings;
mod request_guards;
mod state;

pub fn routes() -> Vec<Route> {
    let mut routes = vec![];

    routes.append(&mut configuration::routes());
    routes.append(&mut cookies::routes());
    routes.append(&mut database::routes());
    routes.append(&mut dynamic_paths::routes());
    routes.append(&mut format::routes());
    routes.append(&mut forwarding::routes());
    routes.append(&mut methods::routes());
    routes.append(&mut query_strings::routes());
    routes.append(&mut request_guards::routes());
    routes.append(&mut state::routes());

    routes
}
