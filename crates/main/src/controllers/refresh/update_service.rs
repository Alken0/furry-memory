use crate::entities::file::{FileInsertForm, FileRepo};
use crate::Database;
use file_system::dir::{Directory, Entry};
use rocket::futures::future::join_all;
use rocket::tokio::task;
use std::convert::TryInto;

pub struct UpdateService {
    to_check: Vec<Directory>,
}

impl UpdateService {
    pub fn new(to_check: &str) -> Self {
        Self {
            to_check: vec![Directory::new(to_check.to_owned())],
        }
    }

    pub async fn clean_run(self, db: Database) {
        self.clean(&db).await;
        self.run(db).await;
    }

    async fn clean(&self, db: &Database) {
        for dir in &self.to_check {
            FileRepo::delete_by_path(db, dir.path()).await;
        }
    }

    async fn run(mut self, db: Database) {
        task::spawn(async move {
            while !self.to_check.is_empty() {
                let content = collect_content(&self.to_check).await;
                self.to_check = content.dirs;
                FileRepo::insert(&db, DirContent::inserts(content.files)).await;
            }
        });
    }
}

#[derive(Default)]
struct DirContent {
    files: Vec<file_system::file::File>,
    dirs: Vec<file_system::dir::Directory>,
}

impl DirContent {
    fn inserts(files: Vec<file_system::file::File>) -> Vec<FileInsertForm> {
        // todo fn(self) does not work because of ownership problems
        files
            .into_iter()
            .filter_map(|f| f.try_into().ok())
            .collect()
    }
}

async fn collect_content(elements: &[Directory]) -> DirContent {
    let async_checks = elements.iter().map(|e| e.elements());

    let entries: Vec<Entry> = join_all(async_checks)
        .await
        .into_iter()
        .filter_map(|e| e.ok())
        .flatten()
        .collect();

    let mut output = DirContent::default();
    entries.into_iter().for_each(|f| match f {
        Entry::File(f) => output.files.push(f),
        Entry::Directory(d) => output.dirs.push(d),
    });

    return output;
}

/*
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
                collect_content(&vec![PathBuf::from("./tests/data")], &DataTypeField::Test).await;
            let expected: ToCheckPaths = vec![
                PathBuf::from("./tests/data/test-file.txt"),
                PathBuf::from("./tests/data/test-file.yml"),
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
            let mut to_check = vec![PathBuf::from("./tests/data")];
            for _ in 0..5 {
                to_check = collect_content(&to_check, &DataTypeField::Video).await.1;
            }
            assert!(to_check.is_empty());
        }
    }
}
 */
