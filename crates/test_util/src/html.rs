use rocket::{http::ContentType, local::asynchronous::LocalResponse};

pub struct HTML(String);
impl HTML {
    pub async fn new(response: LocalResponse<'_>) -> Self {
        assert_eq!(response.content_type(), Some(ContentType::HTML));

        if response.body().is_none() {
            panic!("empty response body")
        }

        Self(response.into_string().await.unwrap())
    }

    pub fn assert_contains(&self, substring: &str) {
        if !self.0.contains(&substring) {
            panic!("html does not contain: {}", substring);
        }
    }

    pub fn assert_has_title(&self, title: &str) {
        let html = format!("<title>{}</title>", title);
        if !self.0.contains(&html) {
            panic!("html does not contain titel: {}", title);
        }
    }

    pub fn assert_charset_utf8(&self) {
        let charset = "<meta charset=\"utf-8\" />";
        if !self.0.contains(&charset) {
            panic!("html does not contain charset: {}", charset);
        }
    }
}
