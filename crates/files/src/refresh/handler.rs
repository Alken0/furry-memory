use super::form::RefreshForm;
use super::template::{RefreshTemplate, TemplateResponse, TemplateResult};
use super::util::Updater;
use base::db::Database;
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

#[cfg(test)]
mod test {
    use crate::test::rocket;
    use base::test::HTML;
    use rocket::{http::Status, local::asynchronous::Client};

    #[rocket::async_test]
    async fn get() {
        let client = Client::tracked(rocket()).await.unwrap();
        let response = client.get("/files/refresh").dispatch().await;

        assert_eq!(response.status(), Status::Ok);

        let body = HTML::new(response).await;
        body.assert_charset();
        body.assert_has_title("Netflex");
    }

    mod post {
        use rocket::http::ContentType;

        use super::*;

        #[rocket::async_test]
        async fn requires_form() {
            let client = Client::tracked(rocket()).await.unwrap();
            let response = client.post("/files/refresh").dispatch().await;

            assert_eq!(response.status(), Status::NotFound);
        }

        #[rocket::async_test]
        async fn invalid_data() {
            let client = Client::tracked(rocket()).await.unwrap();
            let response = client
                .post("/files/refresh")
                .header(ContentType::Form)
                .body("path=%2Ffefl&data_type=x")
                .dispatch()
                .await;

            assert_eq!(response.status(), Status::UnprocessableEntity);
        }

        #[rocket::async_test]
        async fn redirects() {
            let client = Client::tracked(rocket()).await.unwrap();
            let response = client
                .post("/files/refresh")
                .header(ContentType::Form)
                .body("path=./test-data&data_type=Video")
                .dispatch()
                .await;

            //TODO test redirect url
            assert_eq!(response.status(), Status::SeeOther);
        }
    }
}
