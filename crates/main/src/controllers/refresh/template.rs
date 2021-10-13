use super::form::RefreshForm;
use rocket::{
    form::{Context, Contextual, Form},
    http::Status,
};
use rocket_dyn_templates::Template;

pub type TemplateResponse = (Status, Template);
pub type TemplateResult<T> = Result<T, TemplateResponse>;
pub type TemplateForm<'a, T> = Form<Contextual<'a, T>>;

const TEMPLATE: &str = "views/refresh";

pub struct RefreshTemplate;

impl RefreshTemplate {
    pub fn validate<'a, 'b>(
        form: &'b TemplateForm<'a, RefreshForm>,
    ) -> Result<&'b RefreshForm, TemplateResponse>
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

    pub fn render() -> TemplateResponse {
        (Status::Ok, Template::render(TEMPLATE, &Context::default()))
    }
}
