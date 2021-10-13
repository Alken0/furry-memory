use super::form::RefreshForm;
use super::template::{RefreshTemplate, TemplateResponse, TemplateResult};
use super::util::Updater;
use crate::Database;
use rocket::form::{Contextual, Form};
use rocket::response::Redirect;

#[get("/refresh")]
pub async fn get() -> TemplateResponse {
    RefreshTemplate::render()
}

#[post("/refresh", data = "<form>")]
pub async fn post(
    db: Database,
    form: Form<Contextual<'_, RefreshForm>>,
) -> TemplateResult<Redirect> {
    let data = RefreshTemplate::validate(&form)?;

    Updater::new(&(&data.path).into(), &data.data_type)
        .clean_run(db)
        .await;

    return Ok(Redirect::to("/"));
}
