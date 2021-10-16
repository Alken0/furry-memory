mod common;

use common::server::test_client;
use rocket::http::{Header, Status};

mod get {
    use super::*;

    #[rocket::async_test]
    async fn invalid_id_returns_not_found() {
        let client = test_client().await;
        let response = client
            .get("/files/stream/1234")
            .header(Header::new("Content-Range", "0-100/*"))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::NotFound);
    }
}
