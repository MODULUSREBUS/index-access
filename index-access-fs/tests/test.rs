use index_access_fs::IndexAccessFs;
use index_access_storage::IndexAccess;
use tempfile::Builder;
use tokio::test;

#[test]
async fn can_call_new() {
    let dir = Builder::new().prefix("test_iaf").tempdir().unwrap();
    let _storage = IndexAccessFs::new(dir.path()).await.unwrap();
}

#[test]
async fn can_open_buffer() {
    let dir = Builder::new().prefix("test_iaf").tempdir().unwrap();
    let mut storage = IndexAccessFs::new(dir.path()).await.unwrap();
    storage.write(0, b"hello").await.unwrap();
}

#[test]
async fn can_write() {
    let dir = Builder::new().prefix("test_iaf").tempdir().unwrap();
    let mut storage = IndexAccessFs::new(dir.path()).await.unwrap();
    storage
        .write(10, b"hello")
        .await
        .unwrap();
    storage
        .write(20, b"world")
        .await
        .unwrap();
}

#[test]
async fn can_read() {
    let dir = Builder::new().prefix("test_iaf").tempdir().unwrap();
    let mut storage = IndexAccessFs::new(dir.path()).await.unwrap();
    storage
        .write(10, b"hello world")
        .await
        .unwrap();
    let text = storage.read(10).await.unwrap();
    assert_eq!(String::from_utf8(text.to_vec()).unwrap(), "hello world");
}
