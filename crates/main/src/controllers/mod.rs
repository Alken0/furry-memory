mod audios;
mod files;
mod index;
mod refresh;
mod r#static;
mod stream;
mod videos;

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
                videos::list,
                videos::detail,
                files::list,
                audios::list,
                audios::detail
            ],
        )
    })
}
