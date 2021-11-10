use diesel_migrations::embed_migrations;
use rocket::{Build, Rocket};

#[rocket_sync_db_pools::database("diesel")]
pub struct Database(diesel::SqliteConnection);

use rocket::fairing::AdHoc;
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Apply Diesel Config", |rocket| async {
        rocket
            .attach(Database::fairing())
            .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
    })
}

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    // This macro from `diesel_migrations` defines an `embedded_migrations`
    // module containing a function named `run` that runs the migrations in the
    // specified directory, initializing the database.
    embed_migrations!("../../diesel/migrations");

    let conn = Database::get_one(&rocket)
        .await
        .expect("database connection");
    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("diesel migrations");

    rocket
}
