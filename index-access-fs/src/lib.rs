#![forbid(unsafe_code, bad_style, nonstandard_style, future_incompatible)]
#![forbid(rust_2018_idioms, rust_2021_compatibility)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![cfg_attr(test, allow(dead_code))]
#![cfg_attr(test, deny(warnings))]

//! # index-access-fs
//! Indexed read/write from filesystem.

use async_trait::async_trait;
use std::path::{Path, PathBuf};
use tokio::fs::{self, OpenOptions};
use tokio::io::AsyncWriteExt;

use index_access_storage::IndexAccess;

pub use std::io::{Error, ErrorKind};

/// IndexAccessFs.
#[derive(Debug, Clone)]
pub struct IndexAccessFs {
    root: PathBuf,
}
impl IndexAccessFs {
    /// Create new [IndexAccessFs].
    pub async fn new(root: &Path) -> Result<Self, Error> {
        fs::create_dir_all(root).await?;

        Ok(Self {
            root: root.to_path_buf(),
        })
    }
}
#[async_trait]
impl IndexAccess for IndexAccessFs {
    type Error = Error;

    async fn write(&mut self, index: u32, data: &[u8]) -> Result<(), Self::Error> {
        let path = self.root.join(index.to_string());
        let data = data.to_vec();

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
            .await?;
        file.write_all(&data).await?;
        file.sync_all().await?;

        Ok(())
    }

    async fn read(&mut self, index: u32) -> Result<Option<Vec<u8>>, Self::Error> {
        let path = self.root.join(index.to_string());
        match fs::read(&path).await {
            Ok(data) => Ok(Some(data)),
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
        }
    }
}
