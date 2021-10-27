use crate::{entities::file::FileRepo, Database};
use rocket_dyn_templates::tera::Context;
use rocket_dyn_templates::Template;

#[get("/videos")]
pub async fn list(db: Database) -> Template {
    let files = FileRepo::find_all_videos(&db).await;

    let mut context = Context::new();
    context.insert("videos", &files);
    Template::render("views/videos", context.into_json())
}

#[get("/videos/<id>")]
pub async fn detail(db: Database, id: i32) -> Template {
    let video = FileRepo::find_by_id(&db, id).await;

    let mut context = Context::new();
    context.insert("video", &video);
    Template::render("views/video-player", context.into_json())
}
