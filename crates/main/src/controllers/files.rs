use crate::{entities::file::FileRepo, Database};
use rocket_dyn_templates::tera::Context;
use rocket_dyn_templates::Template;

#[get("/files")]
pub async fn list(db: Database) -> Template {
    let files = FileRepo::find_all(&db).await;

    let mut context = Context::new();
    context.insert("files", &files);
    Template::render("views/files", context.into_json())
}
