use rocket::{http::Status, request::FromRequest, request::Outcome, request::Request};
use std::cmp::min;
pub struct Range {
    pub start: i64,
    pub end: i64,
}

impl Range {
    pub fn apply_filesize(&self, file_size: i64) -> Self {
        Self {
            start: min(file_size - 1, self.start),
            end: min(file_size - 1, self.end),
        }
    }
}

// https://datatracker.ietf.org/doc/html/rfc7233#page-13
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Range {
    type Error = String;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let content_range = match req.headers().get_one("Range") {
            Some(s) => s,
            None => return range_not_satisfiable("Missing 'range' in header"),
        };

        let mut values = content_range.split(|c| c == '-' || c == '/');
        let start = match values.next() {
            Some(s) => s.replace("bytes=", ""),
            None => format!("{}", 0),
        };
        let start = match start.parse::<i64>() {
            Ok(o) => o,
            Err(_) => {
                return range_not_satisfiable(&format!("invalid start value: {}", content_range))
            }
        };

        let default = start + 1048576;
        let mut end = match values.next() {
            Some(s) => s.to_string(),
            None => format!("{}", default),
        };
        if end.is_empty() {
            end = format!("{}", default)
        }
        let end = match end.parse::<i64>() {
            Ok(o) => o,
            Err(_) => {
                return range_not_satisfiable(&format!(
                    "invalid end value: \"{}\" ({})",
                    end, content_range
                ))
            }
        };

        if start > end || start < 0 || end < 0 {
            return range_not_satisfiable(&format!("invalid values: {}", content_range));
        }

        return Outcome::Success(Self { start, end });
    }
}

fn range_not_satisfiable<'a, S, E: From<&'a str>>(message: &'a str) -> Outcome<S, E> {
    print!("{}", message);
    return Outcome::Failure((Status::RangeNotSatisfiable, message.into()));
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::*;
    use rocket::http::Header;
    use rocket::local::asynchronous::Client;

    mod from_request {
        use super::*;

        #[rocket::async_test]
        async fn without_leading_bytes_equal() {
            let client = Client::tracked(rocket()).await.unwrap();
            let req = client
                .post("")
                .header(Header::new("Range", "0-49/50"))
                .body("");
            let outcome = Range::from_request(&req).await;

            assert!(outcome.is_success());
            let outcome = outcome.succeeded().unwrap();
            assert_eq!(outcome.start, 0);
            assert_eq!(outcome.end, 49);
        }

        #[rocket::async_test]
        async fn only_start() {
            let client = Client::tracked(rocket()).await.unwrap();
            let req = client
                .post("")
                .header(Header::new("Range", "bytes=0-"))
                .body("");
            let outcome = Range::from_request(&req).await;

            assert!(outcome.is_success());
            let outcome = outcome.succeeded().unwrap();
            assert_eq!(outcome.start, 0);
            assert!(outcome.end > 0);
        }
    }
}
