use rocket::{local::asynchronous::Client, Rocket};

pub async fn mock_client() -> Client {
    return Client::tracked(mock_rocket()).await.unwrap();
}

pub fn mock_rocket() -> Rocket<rocket::Build> {
    let rocket = rocket::build();
    return configure_rocket(rocket);
}

pub fn configure_rocket(rocket: Rocket<rocket::Build>) -> Rocket<rocket::Build> {
    let config = rocket
        .figment()
        .clone()
        .merge(("databases.diesel.url", ":memory:"))
        .merge(("template_dir", "./../../templates"))
        .merge((rocket::Config::LOG_LEVEL, "off"));
    return rocket.configure(config);
}
