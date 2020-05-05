#![feature(proc_macro_hygiene, decl_macro, never_type)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod config;
mod db;
mod error_catchers;
mod fairings;
mod routes;
mod state;

pub fn rocket_ins() -> rocket::Rocket {
    #[allow(unused_variables)]
    let dev_config = config::AppConf::new().dev;
    // rocket::custom(dev_config)

    rocket::ignite()
        .attach(fairings::Counter::new())
        .attach(fairings::adhoc_launch_printer())
        .attach(fairings::adhoc_put_rewriter())
        .attach(fairings::adhoc_assets_dir_config())
        .attach(db::RedisDbConn::fairing())
        .manage(state::HitCount::new())
        .mount("/", routes::routes())
        .register(error_catchers::catchers())
}
