use crate::db::{schema::files, Database};
use crate::types::Id;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, TextExpressionMethods};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "files"]
pub struct File {
    pub id: Id,
    pub name: String,
    pub path: String,
    pub mime: String,
    pub size: i64,
}

impl File {
    pub async fn find_by_id(db: &Database, id: Id) -> Self {
        db.run(move |conn| files::table.find(id).first(conn))
            .await
            .unwrap()
    }

    pub async fn find_all(db: &Database) -> Vec<Self> {
        db.run(move |conn| files::table.order_by(files::name.asc()).load(conn))
            .await
            .unwrap()
    }

    pub async fn insert(db: &Database, files: Vec<Self>) -> usize {
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

impl PartialEq for File {
    // exclude id for testing purposes
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.path == other.path
            && self.mime == other.mime
            && self.size == other.size
    }
}
