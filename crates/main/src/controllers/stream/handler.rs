use super::util::{Chunk, Range};
use crate::entities::File;
use crate::Database;
use base::types::Id;

#[get("/stream/<id>")]
pub async fn get(db: Database, id: Id, range: Option<Range>) -> Chunk {
    let file = File::find_by_id(&db, id).await;

    return match range {
        Some(range) => Chunk::new(&file, &range.apply_filesize(file.size)).await,
        None => Chunk::new(&file, &Range { start: 0, end: 0 }).await,
    };
}
