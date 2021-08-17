pub fn rocket() -> rocket::Rocket<rocket::Build> {
    base::test::server::rocket().attach(crate::stage())
}
