#![forbid(unsafe_code, bad_style, nonstandard_style, future_incompatible)]
#![forbid(rust_2018_idioms, rust_2021_compatibility)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![cfg_attr(test, allow(dead_code))]
#![cfg_attr(test, deny(warnings))]

//! # index-access-memory
//! Read/write indexed block memory.
//!
//! ## Usage
//! ```rust
//! use index_access_storage::IndexAccess;
//! use index_access_memory::IndexAccessMemory;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//! let mut ram = IndexAccessMemory::default();
//! ram.write(0, b"hello world").await.unwrap();
//! let data = ram.read(0).await.unwrap();
//! assert_eq!(data.unwrap(), b"hello world");
//! # Ok(())
//! # }
//! ```

use async_trait::async_trait;
use index_access_storage::IndexAccess;
use std::collections::HashMap;
use std::convert::Infallible;

/// IndexAccessMemory.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct IndexAccessMemory {
    hashmap: HashMap<u32, Vec<u8>>,
}
#[async_trait]
impl IndexAccess for IndexAccessMemory {
    type Error = Infallible;

    async fn write(&mut self, index: u32, data: &[u8]) -> Result<(), Self::Error> {
        self.hashmap.insert(index, data.to_vec());
        Ok(())
    }

    async fn read(&mut self, index: u32) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(self.hashmap.get(&index).map(Clone::clone))
    }
}
