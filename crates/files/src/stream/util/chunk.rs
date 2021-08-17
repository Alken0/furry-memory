use super::range::Range;
use base::db::models::File;
use rocket::http::{Header, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use std::convert::TryInto;
use std::io::Cursor;
use tokio::fs::File as TokioFile;
use tokio::io::{AsyncReadExt, AsyncSeekExt, SeekFrom};

pub struct Chunk {
    start: i64,
    end: i64,
    file_size: i64,
    mime: String,
    content: Vec<u8>,
}

impl Chunk {
    pub async fn new(file: &File, range: &Range) -> Self {
        let content = get_byte_chunk(&file.path, range).await;
        return Self {
            start: range.start,
            end: range.end,
            file_size: file.size,
            mime: file.mime.to_string(),
            content,
        };
    }
}

async fn get_byte_chunk(path: &str, range: &Range) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut file = TokioFile::open(path.to_owned()).await.unwrap();

    file.seek(SeekFrom::Start(
        range.start.try_into().expect("range.start is negativ"),
    ))
    .await
    .unwrap();
    file.take(range.end.try_into().expect("range.end is negativ"))
        .read_to_end(&mut buffer)
        .await
        .unwrap();

    return buffer;
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
