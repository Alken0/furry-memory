mod common;

use common::server::test_client;
use rocket::http::{ContentType, Status};
use test_util::html::HTML;

mod get {
    use super::*;

    #[rocket::async_test]
    async fn test() {
        let client = test_client().await;
        let response = client.get("/refresh").dispatch().await;

        assert_eq!(response.status(), Status::Ok);

        let body = HTML::new(response).await;
        body.assert_charset_utf8();
        body.assert_has_title("Netflex");
    }
}

mod post {
    use super::*;

    #[rocket::async_test]
    async fn requires_form() {
        let client = test_client().await;
        let response = client.post("/refresh").dispatch().await;

        assert_eq!(response.status(), Status::NotFound);
    }

    #[rocket::async_test]
    async fn invalid_data() {
        let client = test_client().await;
        let response = client
            .post("/refresh")
            .header(ContentType::Form)
            .body("path=%2Ffefl&data_type=x")
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[rocket::async_test]
    async fn redirects() {
        let client = test_client().await;
        let response = client
            .post("/refresh")
            .header(ContentType::Form)
            .body("path=./tests/data&data_type=Video")
            .dispatch()
            .await;

        //TODO test redirect url
        assert_eq!(response.status(), Status::SeeOther);
    }
}
