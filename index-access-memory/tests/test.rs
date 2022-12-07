use index_access_memory::IndexAccessMemory;
use index_access_storage::IndexAccess;
use tokio::test;

#[test]
async fn can_create_new() {
    let _ram = IndexAccessMemory::default();
}

#[test]
async fn can_open_buffer() {
    let mut ram = IndexAccessMemory::default();
    ram.write(0, b"hello").await.unwrap();
}

#[test]
async fn can_write() {
    let mut ram = IndexAccessMemory::default();
    ram.write(1, b"hello").await.unwrap();
    ram.write(2, b"world").await.unwrap();
}

#[test]
async fn can_read() {
    let mut ram = IndexAccessMemory::default();
    ram.write(1, b"hello").await.unwrap();
    ram.write(2, b"world").await.unwrap();
    let mut text = ram.read(1).await.unwrap().unwrap();
    text.append(&mut ram.read(2).await.unwrap().unwrap());
    let text = String::from_utf8(text.to_vec()).unwrap();
    assert_eq!(text, "helloworld");
}
