mod diesel;
mod tera;

pub use self::diesel::Database;

use rocket::fairing::AdHoc;
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Apply Config", |rocket| async {
        rocket
            .attach(self::diesel::stage())
            .attach(self::tera::stage())
    })
}
