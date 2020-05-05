// Format

use rocket::Route;
use serde::{Serialize, Deserialize};
use rocket_contrib::json::Json;

pub fn routes() -> Vec<Route> {
    routes![post_json_format, get_json_format]
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    content: String,
    id: usize,
}

// Rocket 将会检查传入 POST 请求的 Content-Type header 是否为 json，并根据结果进行 route 匹配
#[post("/format", format = "json", data = "<message>")]
pub fn post_json_format(message: Json<Message>) -> Option<String> {
    Some(message.id.to_string())
}

// Rocket 将会检查传入 GET 请求的 Accept header 是否为 json，并根据结果进行 route 匹配
#[get("/format/<id>", format = "json")]
pub fn get_json_format(id: usize) -> Option<Json<Message>> {
  Some(Json(Message {
      id: id,
      content: "message from rust".into()
  }))
}
