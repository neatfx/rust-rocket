// Configuration

use std::path::{Path, PathBuf};

use super::super::state::AssetsDir;
use rocket::response::NamedFile;
use rocket::{Route, State};

pub fn routes() -> Vec<Route> {
    routes![assets]
}

#[get("/config/<asset..>")]
fn assets(asset: PathBuf, assets_dir: State<AssetsDir>) -> Option<NamedFile> {
    NamedFile::open(Path::new(&assets_dir.0).join(asset)).ok()
}
