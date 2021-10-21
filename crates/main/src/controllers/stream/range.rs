use file_system::file::Range as FsRange;
use regex::Regex;
use rocket::{http::Status, request::FromRequest, request::Outcome, request::Request};
use std::cmp::max;

const DEFAULT_RANGE: u64 = 1048576;

#[derive(Default)]
pub struct Range(FsRange);

impl Range {
    pub fn start(&self) -> u64 {
        self.0.start()
    }

    /// returns start + offset or None if the number is too big
    pub fn end(&self) -> Option<u64> {
        let offset = self.0.offset().checked_sub(1).unwrap_or(0);
        self.0.start().checked_add(offset)
    }

    pub fn range(&self) -> FsRange {
        self.0
    }

    pub fn apply_file_size(&self, file_size: u64) -> Range {
        Range(self.0.apply_filesize(file_size))
    }
}

// https://datatracker.ietf.org/doc/html/rfc7233#page-13
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Range {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let header = match req.headers().get_one("Range") {
            Some(s) => s,
            None => return range_not_satisfiable("Missing 'range' in header"),
        };

        let re = Regex::new(r"[0-9]+").unwrap();
        let mut reversed_numbers: Vec<u64> = re
            .find_iter(header)
            .map(|e| e.as_str())
            .map(|e| e.parse::<u64>().expect("invalid regex"))
            .collect();
        reversed_numbers.reverse();

        let start = reversed_numbers.pop().unwrap_or(0);
        let end = max(
            start,
            reversed_numbers.pop().unwrap_or(start + DEFAULT_RANGE),
        );

        return Outcome::Success(Self(FsRange::new(start, end - start)));
    }
}

fn range_not_satisfiable<'a, S, E: From<&'a str>>(message: &'a str) -> Outcome<S, E> {
    return Outcome::Failure((Status::RangeNotSatisfiable, message.into()));
}

#[cfg(test)]
mod from_request {
    use super::*;
    use rocket::http::Header;
    use test_util::rocket::mock_client;

    #[rocket::async_test]
    async fn without_leading_bytes_equal() {
        let client = mock_client().await;
        let request = client
            .post("")
            .header(Header::new("Range", "5-49/50"))
            .body("");
        let outcome = Range::from_request(&request).await;

        assert!(outcome.is_success());
        let outcome = outcome.succeeded().unwrap();
        assert_eq!(outcome.start(), 5);
        assert_eq!(outcome.end().unwrap(), 49);
    }

    #[rocket::async_test]
    async fn only_start() {
        let client = mock_client().await;
        let request = client
            .post("")
            .header(Header::new("Range", "bytes=0-"))
            .body("");
        let outcome = Range::from_request(&request).await;

        assert!(outcome.is_success());
        let outcome = outcome.succeeded().unwrap();
        assert_eq!(outcome.start(), 0);
        assert!(outcome.end().unwrap() == DEFAULT_RANGE);
    }
}
