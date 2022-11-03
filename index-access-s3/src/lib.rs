#![forbid(unsafe_code, bad_style, nonstandard_style, future_incompatible)]
#![forbid(rust_2018_idioms, rust_2021_compatibility)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![cfg_attr(test, allow(dead_code))]
#![cfg_attr(test, deny(warnings))]

//! # index-access-s3

use std::error::Error;
use async_trait::async_trait;
use opendal::Operator;
use opendal::services::s3;
use index_access_storage::IndexAccess;

/// IndexAccessS3.
pub struct IndexAccessS3 {
    operator: Operator,
}
impl std::fmt::Debug for IndexAccessS3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IndexAccessS3").finish()
    }
}
impl IndexAccessS3 {
    /// Create new IndexAccessS3.
    pub async fn new(
        root: &str,
        bucket: &str,
        region: &str,
        endpoint: &str,
        access_key: &str,
        secret_key: &str,
        ) -> Result<Self, Box<dyn Error + Send + Sync>>
    {
        let mut builder = s3::Builder::default();
        builder.root(root);
        builder.bucket(bucket);
        builder.region(region);
        builder.endpoint(endpoint);
        builder.access_key_id(access_key);
        builder.secret_access_key(secret_key);

        let operator = Operator::new(builder.build()?);

        Ok(Self {
            operator,
        })
    }
}
#[async_trait]
impl IndexAccess for IndexAccessS3 {

    type Error = Box<dyn Error + Send + Sync>;

    /// Write object to index.
    async fn write(
        &mut self,
        index: String,
        data: &[u8],
        ) -> Result<(), Self::Error>
    {
        let object = self.operator.object(&index);
        object.write(data).await?;
        Ok(())
    }

    /// Attempt to read object at index.
    async fn read(
        &mut self,
        index: String,
        ) -> Result<Vec<u8>, Self::Error>
    {
        let object = self.operator.object(&index);
        let data = object.read().await?;
        Ok(data)
    }
}
