use rocket::fairing::AdHoc;
use rocket_dyn_templates::Template;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Apply Tera Config", |rocket| async {
        rocket.attach(Template::fairing())
    })
}
