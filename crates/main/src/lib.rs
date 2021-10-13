#![allow(clippy::needless_return)]
#![deny(unsafe_code)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod config;
mod controllers;
mod entities;

pub use config::Database;

pub fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(config::stage())
        .attach(controllers::stage())
}
