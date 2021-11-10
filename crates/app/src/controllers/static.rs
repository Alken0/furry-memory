use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

#[get("/static/<path..>")]
pub async fn get(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).await.ok()
}
