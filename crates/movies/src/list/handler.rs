use base::db::{models::File, Database};
use rocket_dyn_templates::tera::Context;
use rocket_dyn_templates::Template;

#[get("/")]
pub async fn get(db: Database) -> Template {
    let mut context = Context::new();
    let files = File::find_all(&db).await;
    context.insert("movies", &files);
    Template::render("views/movies", context.into_json())
}

#[cfg(test)]
mod test {
    use crate::test::rocket;
    use base::test::HTML;
    use rocket::{http::Status, local::asynchronous::Client};

    #[rocket::async_test]
    async fn get() {
        let client = Client::tracked(rocket()).await.unwrap();
        let response = client.get("/movies").dispatch().await;

        assert_eq!(response.status(), Status::Ok);

        let body = HTML::new(response).await;
        body.assert_charset();
        body.assert_has_title("Netflex");
    }
}
