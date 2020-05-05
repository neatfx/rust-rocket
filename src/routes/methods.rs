// Methods

use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![index_handler, a_handler, other::b_handler,]
}

// GET
#[get("/")]
fn index_handler() -> &'static str {
    "Hello, world!"
}

// POST
#[get("/a")] // <- route attribute
fn a_handler() -> &'static str {
    // <- request handler
    "route a"
}

// 嵌套路径（ 需要使用 route![other::b_handler] 的形式进行挂载 ）
mod other {
    #[get("/b")]
    pub fn b_handler() -> &'static str {
        "route b"
    }
}
