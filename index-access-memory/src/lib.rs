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
//! let text = ram.read(0).await.unwrap();
//! assert_eq!(text, b"hello world");
//! # Ok(())
//! # }
//! ```

use anyhow::anyhow;
use async_trait::async_trait;
use index_access_storage::IndexAccess;
use std::collections::HashMap;
use std::error::Error;

/// IndexAccessMemory.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct IndexAccessMemory {
    hashmap: HashMap<u32, Vec<u8>>,
}
#[async_trait]
impl IndexAccess for IndexAccessMemory {
    type Error = Box<dyn Error + Send + Sync>;

    /// Write object to index.
    async fn write(&mut self, index: u32, data: &[u8]) -> Result<(), Self::Error> {
        let data = data.to_vec();
        self.hashmap.insert(index, data);
        Ok(())
    }

    /// Attempt to read object at index.
    async fn read(&mut self, index: u32) -> Result<Vec<u8>, Self::Error> {
        match self.hashmap.get(&index) {
            None => Err(anyhow!("No value for index : {}.", index).into()),
            Some(data) => Ok(data.clone()),
        }
    }
}
