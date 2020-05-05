// Query Strings

use rocket::http::RawStr;
use rocket::request::Form;
use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![
        query,
        query_with_optional_para,
        query_with_multiple_segements,
        optional_item,
    ]
}

// 发送到 /query 的请求至少有一个请求键名 “name” 以及一个内容为 “wave” 的请求片段 ，两者次序不限
// /query?name=John&wave (reordered)
// /query?name=John&wave&id=123 (extra segments)
// /query?id=123&name=John&wave (reordered, extra segments)
// /query?name=Bob&name=John&wave (last value taken)
#[get("/query?wave&<name>")]
fn query(name: &RawStr) -> String {
    format!("{}", name.as_str())
}

// Optional Parameters
#[get("/query_with_optional_para?wave&<name>")]
fn query_with_optional_para(name: Option<String>) -> String {
    name.map(|name| format!("{}", name))
        .unwrap_or_else(|| "No name Parameter".into())
}

// Multiple Segments
#[derive(FromForm, Debug)]
struct User {
    name: String,
    account: usize,
}

// /item?id=100&name=sandal&account=400
#[get("/query_with_multiple_segements?<id>&<user..>")]
fn query_with_multiple_segements(id: usize, user: Form<User>) -> String {
    format!("id - {}, user - {:?}", id, user)
}

// To catch forms that fail to validate
#[get("/optional-item?<id>&<user..>")]
fn optional_item(id: usize, user: Option<Form<User>>) -> String {
    format!("id - {}, user - {:?}", id, user)
}
