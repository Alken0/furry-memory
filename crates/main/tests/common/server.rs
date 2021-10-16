use rocket::local::asynchronous::Client;
use test_util::rocket::configure_rocket;

pub fn test_rocket() -> rocket::Rocket<rocket::Build> {
    return configure_rocket(main::rocket());
}

pub async fn test_client() -> Client {
    return Client::tracked(test_rocket()).await.unwrap();
}
