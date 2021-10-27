use crate::entities::file::File;
use crate::{entities::file::FileRepo, Database};
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
struct ListContext<'a> {
    audios: &'a Vec<File>,
}

#[get("/audios")]
pub async fn list(db: Database) -> Template {
    let files = FileRepo::find_all_audios(&db).await;

    Template::render("views/audios", ListContext { audios: &files })
}

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
struct DetailContext<'a> {
    audio: &'a File,
}

#[get("/audios/<id>")]
pub async fn detail(db: Database, id: i32) -> Template {
    let file = FileRepo::find_by_id(&db, id).await;

    Template::render("views/audio-player", DetailContext { audio: &file })
}
