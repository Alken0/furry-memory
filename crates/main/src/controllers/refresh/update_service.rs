use crate::entities::file::{FileInsertForm, FileRepo};
use crate::Database;
use file_system::dir::{Directory as FsDirectory, Entry};
use file_system::file::File as FsFile;
use rocket::futures::future::join_all;
use rocket::tokio::task;
use std::convert::TryInto;

pub struct UpdateService {
    to_check: Vec<FsDirectory>,
}

impl UpdateService {
    pub fn new(to_check: &str) -> Self {
        println!("{}", to_check);
        Self {
            to_check: vec![FsDirectory::new(to_check.to_owned())],
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
                FileRepo::insert(&db, into_file_insert_form(content.files)).await;
            }
        });
    }
}

#[derive(Default)]
struct DirContent {
    files: Vec<FsFile>,
    dirs: Vec<FsDirectory>,
}

fn into_file_insert_form(files: Vec<FsFile>) -> Vec<FileInsertForm> {
    files
        .into_iter()
        .filter_map(|f| f.try_into().ok())
        .collect()
}

async fn collect_content(elements: &[FsDirectory]) -> DirContent {
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

#[cfg(test)]
mod test {
    use super::*;
    use test_util::functions::assert_vec_equal;

    #[rocket::async_test]
    async fn files_and_dirs() {
        let result = collect_content(&vec![FsDirectory::new("./tests/data".to_owned())]).await;
        let expected_files: Vec<FsFile> = into_fsfile(&vec![
            "./tests/data/test-file.txt",
            "./tests/data/test-file.yml",
        ])
        .await;
        let expected_dirs: Vec<FsDirectory> = into_fsdir(&vec![
            "./tests/data/dir1",
            "./tests/data/dir2",
            "./tests/data/dir3",
        ])
        .await;

        assert_vec_equal(&result.files, &expected_files, "did not find all files");
        assert_vec_equal(&result.dirs, &expected_dirs, "did not find all dirs");
    }

    #[rocket::async_test]
    async fn multiple_dirs_as_input() {
        let input_dirs: Vec<FsDirectory> = into_fsdir(&vec![
            "./tests/data/dir1",
            "./tests/data/dir2",
            "./tests/data/dir3",
        ])
        .await;
        let result = collect_content(&input_dirs).await;
        let expected_files: Vec<FsFile> = into_fsfile(&vec![
            "./tests/data/dir1/test1.txt",
            "./tests/data/dir2/test2.txt",
            "./tests/data/dir3/test3.txt",
        ])
        .await;

        assert_vec_equal(&result.files, &expected_files, "did not find all files");
        assert_vec_equal(&result.dirs, &Vec::new(), "found non existing dirs");
    }

    #[rocket::async_test]
    async fn non_existing_dir() {
        let dir = vec![FsDirectory::new("./tests/data/not_found".to_owned())];
        let result = collect_content(&dir).await;
        assert!(result.files.is_empty());
        assert!(result.dirs.is_empty());
    }

    #[rocket::async_test]
    async fn empty_input() {
        let result = collect_content(&Vec::new()).await;
        assert!(result.files.is_empty());
        assert!(result.dirs.is_empty());
    }

    async fn into_fsfile(paths: &[&str]) -> Vec<FsFile> {
        let mut output = Vec::new();
        for p in paths {
            let file = FsFile::new_from_path(p).await.unwrap();
            output.push(file);
        }
        return output;
    }

    async fn into_fsdir(paths: &[&str]) -> Vec<FsDirectory> {
        let mut output = Vec::new();
        for p in paths {
            let dir = FsDirectory::new(p.to_string());
            output.push(dir);
        }
        return output;
    }
}
