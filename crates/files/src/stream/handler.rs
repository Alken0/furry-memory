use super::util::{Chunk, Range};
use base::db::{models::File, Database};
use base::types::Id;

#[get("/stream/<id>")]
pub async fn get(db: Database, id: Id, range: Option<Range>) -> Chunk {
    let file = File::find_by_id(&db, id).await;

    return match range {
        Some(range) => Chunk::new(&file, &range.apply_filesize(file.size)).await,
        None => Chunk::new(&file, &Range { start: 0, end: 0 }).await,
    };
}

#[cfg(test)]
mod test {
    use crate::test::*;
    use rocket::local::asynchronous::Client;

    mod get {
        use super::*;
        use rocket::http::{Header, Status};

        #[rocket::async_test]
        async fn invalid_id_returns_not_found() {
            let client = Client::tracked(rocket()).await.unwrap();
            let response = client
                .get("/files/stream/1234")
                .header(Header::new("Content-Range", "0-100/*"))
                .dispatch()
                .await;

            assert_eq!(response.status(), Status::NotFound);
        }
    }
}
