use super::schema::files;
use crate::Database;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, TextExpressionMethods};
use rocket::serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct File {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub mime: String,
    pub size: i64,
}

impl File {
    pub fn size(&self) -> std::io::Result<u64> {
        self.size.try_into().map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::Interrupted,
                format!(
                    "invalid value ({}) for saved file with id {} ",
                    self.size, self.id
                ),
            )
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "files"]
pub struct FileInsertForm {
    pub name: String,
    pub path: String,
    pub mime: String,
    pub size: i64,
}

pub struct FileRepo;

impl FileRepo {
    pub async fn find_by_id(db: &Database, id: i32) -> File {
        db.run(move |conn| files::table.find(id).first(conn))
            .await
            .unwrap()
    }

    pub async fn find_all(db: &Database) -> Vec<File> {
        db.run(move |conn| files::table.order_by(files::name.asc()).load(conn))
            .await
            .unwrap()
    }

    pub async fn insert(db: &Database, files: Vec<FileInsertForm>) -> usize {
        db.run(move |conn| {
            diesel::insert_into(files::table)
                .values(&*files)
                .execute(conn)
        })
        .await
        .unwrap()
    }

    pub async fn delete_by_path(db: &Database, path: String) -> usize {
        db.run(move |conn| {
            diesel::delete(files::table)
                .filter(files::path.like(format!("{}%", path)))
                .execute(conn)
        })
        .await
        .unwrap()
    }
}

impl TryFrom<file_system::file::File> for FileInsertForm {
    type Error = std::io::Error;

    fn try_from(value: file_system::file::File) -> Result<Self, Self::Error> {
        Ok(Self {
            mime: value.mime()?,
            name: value.name(),
            path: value.path(),
            size: try_convert_size(&value)?,
        })
    }
}

fn try_convert_size(value: &file_system::file::File) -> std::io::Result<i64> {
    i64::try_from(value.size()).map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("file '{}' is too big to store in database", value.path()),
        )
    })
}
