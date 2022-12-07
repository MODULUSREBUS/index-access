#![forbid(unsafe_code, bad_style, nonstandard_style, future_incompatible)]
#![forbid(rust_2018_idioms, rust_2021_compatibility)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![cfg_attr(test, allow(dead_code))]
#![cfg_attr(test, deny(warnings))]

//! # index-access-s3

use async_trait::async_trait;
use index_access_storage::IndexAccess;
use opendal::services::s3;
use opendal::Operator;

pub use opendal::{Error, ErrorKind};

/// IndexAccessS3.
#[derive(Clone)]
pub struct IndexAccessS3 {
    operator: Operator,
}
impl std::fmt::Debug for IndexAccessS3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IndexAccessS3").finish()
    }
}
impl IndexAccessS3 {
    /// Create new [IndexAccessS3].
    pub fn new(
        root: &str,
        bucket: &str,
        region: &str,
        endpoint: &str,
        access_key: &str,
        secret_key: &str,
    ) -> Result<Self, Error> {
        let accessor = s3::Builder::default()
            .root(root)
            .bucket(bucket)
            .region(region)
            .endpoint(endpoint)
            .access_key_id(access_key)
            .secret_access_key(secret_key)
            .build()?;
        let operator = Operator::new(accessor);

        Ok(Self { operator })
    }
}
#[async_trait]
impl IndexAccess for IndexAccessS3 {
    type Error = Error;

    async fn write(&mut self, index: u32, data: &[u8]) -> Result<(), Self::Error> {
        let object = self.operator.object(&index.to_string());
        object.write(data).await?;
        Ok(())
    }

    async fn read(&mut self, index: u32) -> Result<Option<Vec<u8>>, Self::Error> {
        let object = self.operator.object(&index.to_string());
        match object.read().await {
            Ok(data) => Ok(Some(data)),
            Err(e) if e.kind() == ErrorKind::ObjectNotFound => Ok(None),
            Err(e) => Err(e),
        }
    }
}
