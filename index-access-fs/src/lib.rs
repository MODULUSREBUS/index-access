#![forbid(unsafe_code, bad_style, nonstandard_style, future_incompatible)]
#![forbid(rust_2018_idioms, rust_2021_compatibility)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![cfg_attr(test, allow(dead_code))]
#![cfg_attr(test, deny(warnings))]

//! # index-access-fs
//! Read/write fs.

use anyhow::Result;
use std::path::{PathBuf, Path};
use std::error::Error;
use async_trait::async_trait;
use tokio::fs::{self, OpenOptions};
use tokio::io::AsyncWriteExt;

use index_access_storage::IndexAccess;

/// IndexAccessFs.
#[derive(Debug)]
pub struct IndexAccessFs {
    root: PathBuf,
}
impl IndexAccessFs {
    /// Returns [IndexAccessFs].
    pub async fn new(root: &Path) -> Result<Self> {
        fs::create_dir_all(root).await?;

        Ok(Self {
            root: root.to_path_buf(),
        })
    }
}
#[async_trait]
impl IndexAccess for IndexAccessFs {

    type Error = Box<dyn Error + Send + Sync>;

    /// Write object to index.
    async fn write(
        &mut self,
        index: String,
        data: &[u8],
        ) -> Result<(), Self::Error>
    {
        let path = self.root.join(index);
        let data = data.to_vec();

        let mut file = OpenOptions::new()
            .create(true)
            .read(false)
            .write(true)
            .open(&path)
            .await?;
        file.write_all(&data).await?;
        file.set_len(data.len() as u64).await?;
        file.sync_all().await?;

        Ok(())
    }

    /// Attempt to read object at index.
    async fn read(
        &mut self,
        index: String,
        ) -> Result<Vec<u8>, Self::Error>
    {
        let path = self.root.join(index);

        let data = fs::read(&path).await?;

        Ok(data)
    }
}
