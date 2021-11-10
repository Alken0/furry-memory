#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    main::rocket()
}
