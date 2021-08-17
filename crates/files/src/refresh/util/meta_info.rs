use super::path_info::PathInfo;
use base::db::models::File;
use base::types::Id;
use std::convert::TryFrom;
use std::{
    fs::Metadata,
    io::{Error, ErrorKind},
};
use tokio::fs::metadata;

pub struct MetaInfo {
    meta: Metadata,
    path: PathInfo,
}

impl MetaInfo {
    pub async fn new(path: &PathInfo) -> Result<Self, Error> {
        return Ok(Self {
            meta: metadata(&path.path()).await?,
            path: path.to_owned(),
        });
    }

    pub fn is_file(&self) -> bool {
        self.meta.is_file()
    }

    pub fn is_dir(&self) -> bool {
        self.meta.is_dir()
    }

    pub fn file_size(&self) -> Result<i64, Error> {
        let converted = i64::try_from(self.meta.len());
        return match converted {
            Ok(o) => Ok(o),
            Err(_) => Err(Error::new(
                ErrorKind::Unsupported,
                format!(
                    "file is too big, has to be in range of i64 (path: {})",
                    self.path.path()
                ),
            )),
        };
    }
}

impl TryFrom<&MetaInfo> for File {
    type Error = std::io::Error;

    fn try_from(value: &MetaInfo) -> Result<File, Self::Error> {
        Ok(File {
            id: Id::random(),
            mime: value.path.mime()?,
            name: value.path.name(),
            path: value.path.path(),
            size: value.file_size()?,
        })
    }
}
