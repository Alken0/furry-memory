use super::range::Range;
use file_system::file::File as FsFile;
use rocket::http::{Header, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use std::io::Cursor;
use std::io::Result;

pub struct Chunk {
    start: u64,
    end: u64,
    file_size: u64,
    mime: String,
    content: Vec<u8>,
}

impl Chunk {
    pub async fn new(file: &FsFile, range: &Range) -> Result<Self> {
        Ok(Self {
            start: range.start(),
            end: range.end().unwrap_or_default(),
            file_size: file.size(),
            mime: file.mime()?,
            content: file.chunk(&range.range()).await?,
        })
    }
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for Chunk {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .status(Status::PartialContent)
            .header(Header::new("Content-Type", self.mime))
            .header(Header::new("Accept-Ranges", "bytes"))
            .header(Header::new(
                "Content-Range",
                format!("bytes {}-{}/{}", self.start, self.end, self.file_size),
            ))
            .sized_body(self.content.len(), Cursor::new(self.content))
            .ok()
    }
}
