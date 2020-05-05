// Request > Body Data > Streaming

use super::super::state::AssetsDir;
use rocket::{response::Debug, Data};
use rocket::{Route, State};
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

pub fn routes() -> Vec<Route> {
    routes![upload, get, delete]
}

#[post("/body-data/streaming", format = "plain", data = "<data>")]
fn upload(data: Data, assets_dir: State<AssetsDir>) -> Result<String, Debug<io::Error>> {
    data.stream_to_file(Path::new(&assets_dir.0).join("upload.txt"))
        .map(|n| n.to_string())
        .map_err(Debug)
}

#[get("/body-data/streaming/<file..>")]
fn get(file: PathBuf, assets_dir: State<AssetsDir>) -> String {
    let upload_file = Path::new(&assets_dir.0).join(file);
    let mut file_contents = String::new();
    let mut file = File::open(&upload_file).expect("open upload.txt file");
    file.read_to_string(&mut file_contents)
        .expect("read upload.txt");
    file_contents
}

#[delete("/body-data/streaming/<file..>")]
fn delete(file: PathBuf, assets_dir: State<AssetsDir>) -> Result<String, ()> {
    let upload_file = Path::new(&assets_dir.0).join(file);
    match fs::remove_file(&upload_file) {
        Ok(_) => Ok("deleted".into()),
        Err(e) => Ok(e.to_string()),
    }
}
