use rocket::{fs::NamedFile, get, routes};
use rocket_dyn_templates::Template;
use std::path::{Path, PathBuf};

pub fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(Template::fairing())
        .attach(crate::db::stage())
        .mount("/", routes![assets])
}

#[get("/assets/<path..>")]
pub async fn assets(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("templates/assets/").join(path))
        .await
        .ok()
}
