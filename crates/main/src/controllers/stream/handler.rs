use super::chunk::Chunk;
use super::range::Range;
use crate::entities::file::FileRepo;
use crate::Database;

#[get("/stream/<id>")]
pub async fn get(db: Database, id: i32, range: Option<Range>) -> Result<Chunk, std::io::Error> {
    let file = FileRepo::find_by_id(&db, id).await;
    let size = file.size()?;

    let fs_file = file_system::file::File::new(file.path.to_owned(), size);

    let response = match range {
        Some(range) => Chunk::new(&fs_file, &range).await?,
        None => Chunk::new(&fs_file, &Range::default()).await?,
    };
    return Ok(response);
}
