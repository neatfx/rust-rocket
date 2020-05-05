// Cookies
// Cookies 是框架内置的 Request Guard，因此可以作为参数类型在 handler 中直接使用
// 支持 Read、Set、Remove 以及 Private Cookies

use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![
        send_cookie_to_client,
        set_cookie_with_posted_data,
        get_cookie_from_client,
        send_private_cookie_to_client,
        parse_private_cookie_from_client,
        remove_private_cookie,
    ]
}

#[derive(FromForm)]
struct Message {
    message: String,
}

// 发送 cookie 到客户端
#[get("/cookies")]
fn send_cookie_to_client(mut cookies: Cookies) -> Redirect {
    cookies.add(Cookie::new("message", "cookie from server"));
    Redirect::to("/cookies/get")
}

// 发送 cookie 到客户端（ 使用客户端提交的 POST 数据 ）
#[post("/cookies", data = "<message>")]
fn set_cookie_with_posted_data(mut cookies: Cookies, message: Form<Message>) -> Redirect {
    cookies.add(Cookie::new("message", message.into_inner().message));
    Redirect::to("/cookies/get")
}

// 解析客户端请求携带的 cookie
#[get("/cookies/get")]
fn get_cookie_from_client(cookies: Cookies) -> Option<String> {
    match cookies.get("message") {
        Some(cookie) => Some(cookie.value().to_string()),
        None => Some("No cookie from client".into()),
    }
}

// 解析 private cookie
// 需要在 Cargo.toml 中启用 rocket 的 private-cookies 功能支持
#[get("/cookies/private")]
fn send_private_cookie_to_client(mut cookies: Cookies) -> Option<String> {
    // 真实应用代码中，应当从数据库中获取 user_id，并放入 private-cookie 中发送
    cookies.add_private(Cookie::new("user_id", "rust"));
    Some("Cookie sended to client".into())
}

// 获取客户端 private cookie 并解析
// FIX: get_private 方法仅作用于 user_id_p
#[get("/cookies/private/parse")]
fn parse_private_cookie_from_client(mut cookies: Cookies) -> Option<String> {
    // println!("xxxx {:#?}", cookies);··
    // println!("aaaa {:#?}", cookies.get("user_id"));
    // println!("ssss {:#?}", cookies.get_private("user_id"));

    match cookies.get_private("user_id_p") {
        Some(cookie) => Some(cookie.value().to_string()),
        None => Some("No private cookie from client".into()),
    }
}

// 移除 private cookie
#[post("/cookies/private/remove")]
fn remove_private_cookie(mut cookies: Cookies) -> Option<String> {
    cookies.remove_private(Cookie::named("user_id"));
    Some("Successfully removed private cookie.".into())
}
