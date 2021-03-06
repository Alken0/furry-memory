use rocket::form;
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, FromForm)]
pub struct RefreshForm {
    pub path: PathField,
    pub data_type: DataTypeField,
}

#[derive(Debug, Clone, FromFormField)]
pub enum DataTypeField {
    Video,
    #[cfg(test)]
    Test,
}

#[derive(Debug)]
pub struct PathField(pub String);

#[rocket::async_trait]
impl<'r> form::FromFormField<'r> for PathField {
    fn from_value(field: form::ValueField<'r>) -> form::Result<'r, Self> {
        let string = field.value.to_string();
        let path = PathBuf::from_str(&string).map_err(|_| validation_error("ill-formatted"))?;
        if !path.exists() {
            return Err(validation_error("path does not exist"));
        }
        return Ok(Self(string));
    }

    async fn from_data(field: form::DataField<'r, '_>) -> form::Result<'r, Self> {
        Err(field.unexpected().into())
    }
}

fn validation_error(message: &str) -> form::Errors {
    form::Errors::from(form::Error::validation(message))
}
