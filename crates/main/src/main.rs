#![allow(clippy::needless_return)]
#![deny(unsafe_code)]

use rocket::response::Redirect;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    base::server::rocket()
        .attach(files::stage())
		.attach(movies::stage())
        .mount("/", routes![index])
}

#[get("/")]
pub async fn index() -> Redirect {
    Redirect::to("/movies")
}
