use rocket::config::Config;

mod dev;

pub struct AppConf {
    pub dev: Config,
    // prod: Config,
}

impl AppConf {
    pub fn new() -> AppConf {
        AppConf { dev: dev::build() }
    }
}
