use rocket::response::Redirect;

#[get("/")]
pub async fn get() -> Redirect {
    Redirect::to("/videos")
}
