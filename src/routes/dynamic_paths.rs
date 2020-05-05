//  Dynamic Paths

use rocket::http::RawStr;
use rocket::response::NamedFile;
use rocket::Route;
use std::path::{Path, PathBuf};

pub fn routes() -> Vec<Route> {
    routes![dyn_handler, dyn_n_handler, files,]
}

// 动态路径
#[get("/dyn/<name>")]
fn dyn_handler(name: &RawStr) -> String {
    format!("{}", name.as_str())
}

// 动态路径可包含任意数量片段
#[get("/dyn-n/<seg1>/<seg2>/<seg3>")]
fn dyn_n_handler(seg1: String, seg2: u8, seg3: bool) -> String {
    if seg3 {
        format!("{}-{}", seg1, seg2)
    } else {
        format!("seg3 is {}", seg3)
    }
}

// Multiple Segments ( 处理静态文件 )
#[get("/file/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}
