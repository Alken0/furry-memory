use crate::{entities::File, Database};
use rocket_dyn_templates::tera::Context;
use rocket_dyn_templates::Template;

#[get("/movies")]
pub async fn get(db: Database) -> Template {
    let mut context = Context::new();
    let files = File::find_all(&db).await;
    context.insert("movies", &files);
    Template::render("views/movies", context.into_json())
}
