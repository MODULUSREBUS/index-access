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
//! let mut ram = IndexAccessMemory::new();
//! ram.write("a".to_string(), b"hello world").await.unwrap();
//! let text = ram.read("a".to_string()).await.unwrap();
//! assert_eq!(text, b"hello world");
//! # Ok(())
//! # }
//! ```

use anyhow::anyhow;
use std::collections::HashMap;
use std::error::Error;
use async_trait::async_trait;
use index_access_storage::IndexAccess;

/// IndexAccessMemory.
#[derive(Debug)]
pub struct IndexAccessMemory {
    hashmap: HashMap<String, Vec<u8>>,
}
impl IndexAccessMemory {
    /// Create new IndexAccessMemory.
    pub fn new() -> Self {
        Self {
            hashmap: HashMap::new(),
        }
    }
}
#[async_trait]
impl IndexAccess for IndexAccessMemory {

    type Error = Box<dyn Error + Send + Sync>;

    /// Write object to index.
    async fn write(
        &mut self,
        index: String,
        data: &[u8],
        ) -> Result<(), Self::Error>
    {
        let data = data.to_vec();
        self.hashmap.insert(index, data);
        Ok(())
    }

    /// Attempt to read object at index.
    async fn read(
        &mut self,
        index: String,
        ) -> Result<Vec<u8>, Self::Error>
    {
        match self.hashmap.get(&index) {
            None => Err(anyhow!("No value for index : {}.", index).into()),
            Some(data) => Ok(data.clone()),
        }
    }
}
