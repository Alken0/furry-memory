use file_system::dir::{Directory, Entry};
use futures::future::join_all;
use test_util::functions::assert_vec_equal;

#[tokio::test]
async fn elements() {
    let path = "./tests/data".to_owned();
    let dir = Directory::new(path).elements().await.unwrap();

    let expected = join_all(vec![
        new_entry("./tests/data/text.txt"),
        new_entry("./tests/data/dir"),
    ])
    .await;

    assert_vec_equal(&dir, &expected, "");
}

async fn new_entry(path: &str) -> Entry {
    Entry::new(path.to_owned()).await.unwrap()
}

#[tokio::test]
async fn invalid_path_is_error() {
    let path = "./tests/not_found".to_owned();
    let elements = Directory::new(path).elements().await;

    assert!(elements.is_err());
}
