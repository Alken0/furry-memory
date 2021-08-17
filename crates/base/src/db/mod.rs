mod database;
pub mod models;
mod schema;
pub use database::Database;

use diesel_migrations::embed_migrations;
use rocket::fairing::AdHoc;
use rocket::{Build, Rocket};

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
        rocket
            .attach(Database::fairing())
            .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
    })
}

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    // This macro from `diesel_migrations` defines an `embedded_migrations`
    // module containing a function named `run` that runs the migrations in the
    // specified directory, initializing the database.
    embed_migrations!("db/migrations");

    let conn = Database::get_one(&rocket)
        .await
        .expect("database connection");
    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("diesel migrations");

    rocket
}
