use tokio::test;
use index_access_storage::IndexAccess;
use index_access_memory::IndexAccessMemory;

#[test]
async fn can_call_new() {
  let _ram = IndexAccessMemory::new();
}

#[test]
async fn can_open_buffer() {
  let mut ram = IndexAccessMemory::new();
  ram.write(0.to_string(), b"hello").await.unwrap();
}

#[test]
async fn can_write() {
  let mut ram = IndexAccessMemory::new();
  ram.write("a".to_owned(), b"hello").await.unwrap();
  ram.write("b".to_owned(), b" world").await.unwrap();
}

#[test]
async fn can_read() {
  let mut ram = IndexAccessMemory::new();
  ram.write("a".to_owned(), b"hello").await.unwrap();
  ram.write("b".to_owned(), b" world").await.unwrap();
  let mut text = ram.read("a".to_owned()).await.unwrap();
  text.append(&mut ram.read("b".to_owned()).await.unwrap());
  let text = String::from_utf8(text.to_vec()).unwrap();
  assert_eq!(text, "hello world");
}
