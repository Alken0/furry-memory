use rocket::local::asynchronous::Client;

pub async fn mock_client() -> Client {
    let rocket = rocket::build();
    let client = Client::tracked(rocket).await.unwrap();
    return client;
}
