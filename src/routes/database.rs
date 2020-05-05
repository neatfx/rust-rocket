use crate::rocket_contrib::databases::redis::Commands;
use rocket::Route;

use super::super::db::RedisDbConn;

pub fn routes() -> Vec<Route> {
    routes![get_logs,]
}

#[get("/database")]
fn get_logs(conn: RedisDbConn) -> Result<String, rocket_contrib::databases::redis::RedisError> {
    conn.set("my_key", 42)?;
    conn.get("my_key")
}
