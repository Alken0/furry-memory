mod index;
mod movies;
mod refresh;
mod r#static;
mod stream;

use rocket::fairing::AdHoc;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Attach Controllers", |rocket| async {
        rocket.mount(
            "/",
            routes![
                index::get,
                r#static::get,
                stream::get,
                refresh::get,
                refresh::post,
                movies::get
            ],
        )
    })
}
