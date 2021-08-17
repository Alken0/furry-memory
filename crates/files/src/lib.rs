#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;
mod refresh;
mod stream;
#[cfg(test)]
mod test;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Stage crate 'Files'", |rocket| async move {
        rocket.mount("/files", routes![stream::get, refresh::post, refresh::get])
    })
}
