use rocket::Request;

// Catcher 接受 0 或 1 个参数，且必须是 &Request 类型
#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("'{}' is not a valid path.", req.uri())
}
