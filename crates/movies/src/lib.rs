#[macro_use]
extern crate rocket;

use rocket::{fairing::AdHoc, routes};
mod list;
#[cfg(test)]
mod test;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Stage crate 'Movies'", |rocket| async move {
        rocket.mount("/movies", routes![list::get])
    })
}
