use super::super::form::DataTypeField;
use super::{MetaInfo, PathInfo};
use crate::entities::File;
use crate::Database;
use rocket::futures::future::join_all;
use std::convert::TryFrom;
use std::path::{Path, PathBuf};
use tokio::fs::read_dir;

type FilePaths = Vec<File>;
type ToCheckPaths = Vec<PathBuf>;

#[derive(Clone)]
pub struct Updater {
    to_check: ToCheckPaths,
    data_type: DataTypeField,
}

impl Updater {
    pub fn new(to_check: &PathBuf, data_type: &DataTypeField) -> Self {
        Self {
            to_check: vec![to_check.to_owned()],
            data_type: data_type.to_owned(),
        }
    }

    pub async fn clean_run(self, db: Database) {
        self.clean(&db).await;
        self.run(db).await;
    }

    async fn clean(&self, db: &Database) {
        for path in &self.to_check {
            File::delete_by_path(db, PathInfo::new(path).path()).await;
        }
    }

    async fn run(mut self, db: Database) {
        tokio::task::spawn(async move {
            while !self.to_check.is_empty() {
                let (values, left_over) = collect_content(&self.to_check, &self.data_type).await;
                File::insert(&db, values).await;
                self.to_check = left_over;
            }
        });
    }
}

async fn collect_content(
    elements: &ToCheckPaths,
    data_type: &DataTypeField,
) -> (FilePaths, ToCheckPaths) {
    let async_checks = elements.iter().map(|e| async move {
        let path_info = PathInfo::new(e);

        if !data_type.valid(&path_info.path()) {
            return (Vec::new(), extract_elements_inside_dir(e).await);
        }

        let meta_info = match MetaInfo::new(&path_info).await {
            Ok(o) => o,
            Err(_) => return (Vec::new(), Vec::new()),
        };

        if meta_info.is_file() {
            return match File::try_from(&meta_info) {
                Ok(o) => (vec![o], Vec::new()),
                Err(_) => (Vec::new(), Vec::new()),
            };
        }

        return (Vec::new(), Vec::new());
    });

    return join_all(async_checks).await.into_iter().fold(
        (Vec::new(), Vec::new()),
        |accumulator, element| {
            (
                [&accumulator.0[..], &element.0[..]].concat(),
                [&accumulator.1[..], &element.1[..]].concat(),
            )
        },
    );
}

async fn extract_elements_inside_dir(path: &Path) -> Vec<PathBuf> {
    let mut entries = match read_dir(&path).await {
        Ok(o) => o,
        Err(_) => return Vec::new(),
    };

    let mut output: Vec<PathBuf> = Vec::new();
    while let Ok(Some(entry)) = entries.next_entry().await {
        output.push(entry.path());
    }
    return output;
}

#[cfg(test)]
mod test {
    use super::*;
    use test_util::functions::assert_vec_equal;

    mod collect_content {
        use super::*;

        #[rocket::async_test]
        async fn correct_return_position() {
            // test: read dir -> identify all elements as to be checked
            let result =
                collect_content(&vec![PathBuf::from("./test-data")], &DataTypeField::Test).await;
            let expected: ToCheckPaths = vec![
                PathBuf::from("./test-data/test-file.txt"),
                PathBuf::from("./test-data/test-file.yml"),
            ];

            assert!(result.0.is_empty());
            assert_vec_equal(&result.1, &expected, "could not find elements in dir");

            // test: read files: identify input as files and return them correctly
            let mut expected_filepaths: FilePaths = Vec::new();
            for e in &expected {
                let path_info = MetaInfo::new(&PathInfo::new(e)).await.unwrap();
                expected_filepaths.push(File::try_from(&path_info).unwrap())
            }
            let result = collect_content(&expected, &DataTypeField::Test).await;
            assert!(result.1.is_empty());
            assert_vec_equal(
                &result.0,
                &expected_filepaths,
                "could not identify elements as files",
            );
        }

        #[rocket::async_test]
        async fn terminates() {
            // ATTENTION: fails if folder-depth is more than 5

            // test: empty input returns empty result
            assert_eq!(
                (Vec::new(), Vec::new()),
                collect_content(&Vec::new(), &DataTypeField::Video).await
            );

            // test: to_check will turn empty
            let mut to_check = vec![PathBuf::from("./src")];
            for _ in 0..5 {
                to_check = collect_content(&to_check, &DataTypeField::Video).await.1;
            }
            assert!(to_check.is_empty());
        }
    }
}
