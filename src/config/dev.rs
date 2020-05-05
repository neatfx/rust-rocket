use rocket::config::{Config, Environment, Value};
use std::collections::HashMap;

#[allow(dead_code)]
pub fn build() -> Config {
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    database_config.insert("url", Value::from("redis://127.0.0.1"));
    databases.insert("redis_logs", Value::from(database_config));

    Config::build(Environment::Development)
        .address("localhost")
        .port(8000)
        .extra("databases", databases)
        .finalize()
        .unwrap()
}
