use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Binary;
use diesel::sqlite::Sqlite;
use rocket::request::FromParam;
use rocket::serde;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::prelude::*;
use std::str::FromStr;

#[derive(
    Debug,
    Clone,
    Copy,
    FromSqlRow,
    AsExpression,
    Hash,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(crate = "rocket::serde")]
#[sql_type = "Binary"]
pub struct Id(uuid::Uuid);

impl Id {
    pub fn random() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl From<Id> for uuid::Uuid {
    fn from(s: Id) -> Self {
        s.0
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromSql<Binary, Sqlite> for Id {
    fn from_sql(bytes: Option<&<Sqlite as Backend>::RawValue>) -> deserialize::Result<Self> {
        let bytes = not_none!(bytes);
        uuid::Uuid::from_slice(bytes.read_blob())
            .map(Id)
            .map_err(|e| e.into())
    }
}

impl ToSql<Binary, Sqlite> for Id {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> serialize::Result {
        out.write_all(self.0.as_bytes())
            .map(|_| IsNull::No)
            .map_err(Into::into)
    }
}

impl<'a> FromParam<'a> for Id {
    type Error = uuid::Error;
    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        uuid::Uuid::from_str(param).map(Id)
    }
}
