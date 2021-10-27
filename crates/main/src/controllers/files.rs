use crate::entities::file::File;
use crate::{entities::file::FileRepo, Database};
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
struct Context<'a> {
    files: &'a Vec<File>,
}

#[get("/files")]
pub async fn list(db: Database) -> Template {
    let files = FileRepo::find_all(&db).await;

    Template::render("views/files", Context { files: &files })
}
