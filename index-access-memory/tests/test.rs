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
    ram.write(0.to_string(), b"hello").await.unwrap();
}

#[test]
async fn can_write() {
    let mut ram = IndexAccessMemory::default();
    ram.write("a".to_owned(), b"hello").await.unwrap();
    ram.write("b".to_owned(), b" world").await.unwrap();
}

#[test]
async fn can_read() {
    let mut ram = IndexAccessMemory::default();
    ram.write("a".to_owned(), b"hello").await.unwrap();
    ram.write("b".to_owned(), b" world").await.unwrap();
    let mut text = ram.read("a".to_owned()).await.unwrap();
    text.append(&mut ram.read("b".to_owned()).await.unwrap());
    let text = String::from_utf8(text.to_vec()).unwrap();
    assert_eq!(text, "hello world");
}
