use crate::{
    entities::file::{File, FileRepo},
    Database,
};
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
struct ListContext<'a> {
    videos: &'a Vec<File>,
}

#[get("/videos")]
pub async fn list(db: Database) -> Template {
    let files = FileRepo::find_all_videos(&db).await;

    Template::render("views/videos", ListContext { videos: &files })
}

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
struct DetailContext<'a> {
    video: &'a File,
}

#[get("/videos/<id>")]
pub async fn detail(db: Database, id: i32) -> Template {
    let file = FileRepo::find_by_id(&db, id).await;

    Template::render("views/video-player", DetailContext { video: &file })
}
