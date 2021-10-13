mod common;

use common::server::test_rocket;
use rocket::{http::Status, local::asynchronous::Client};
use test_util::html::HTML;

mod get {
    use super::*;

    #[rocket::async_test]
    async fn test() {
        let client = Client::tracked(test_rocket()).await.unwrap();
        let response = client.get("/movies").dispatch().await;

        assert_eq!(response.status(), Status::Ok);

        let body = HTML::new(response).await;
        body.assert_charset_utf8();
        body.assert_has_title("Netflex");
    }
}
