use rocket::Config;

pub fn rocket() -> rocket::Rocket<rocket::Build> {
    let rocket = crate::server::rocket();
    let config = rocket
        .figment()
        .clone()
        .merge(("template_dir", "./../../templates"))
        .merge((Config::LOG_LEVEL, "off"))
        .merge(("databases.diesel.url", ":memory:"));
    return rocket.configure(config);
}
