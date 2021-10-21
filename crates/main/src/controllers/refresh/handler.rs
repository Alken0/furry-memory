use super::form::RefreshForm;
use super::update_service::UpdateService;
use crate::Database;
use rocket::form::{Context, Contextual, Form};
use rocket::http::Status;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;

const TEMPLATE: &str = "views/refresh";

#[get("/refresh")]
pub async fn get() -> Template {
    Template::render(TEMPLATE, &Context::default())
}

#[post("/refresh", data = "<form>")]
pub async fn post(
    db: Database,
    form: Form<Contextual<'_, RefreshForm>>,
) -> Result<Redirect, (Status, Template)> {
    let data = validate_form(&form)?;

    UpdateService::new(&data.path.0).clean_run(db).await;

    return Ok(Redirect::to("/"));
}

pub fn validate_form<'a, 'b>(
    form: &'b Form<Contextual<'a, RefreshForm>>,
) -> Result<&'b RefreshForm, (Status, Template)>
where
    'a: 'b,
{
    let data = form.value.as_ref().ok_or_else(|| {
        let status = form.context.status();
        let template = Template::render(TEMPLATE, &form.context);
        return (status, template);
    })?;
    return Ok(data);
}
