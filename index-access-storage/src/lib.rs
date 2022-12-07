#![forbid(unsafe_code, bad_style, nonstandard_style, future_incompatible)]
#![forbid(rust_2018_idioms, rust_2021_compatibility)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![cfg_attr(test, allow(dead_code))]
#![cfg_attr(test, deny(warnings))]

//! Abstract interface to implement "index-access" object storage.
//! This module forms a shared interface for reading and writing objects
//! to different backends.

use async_trait::async_trait;

/// The `IndexAccess` trait allows for reading from and writing to a
/// index-accessible storage of objects.
#[async_trait]
pub trait IndexAccess {
    /// An error.
    type Error;

    /// Write bytes at an offset to the backend.
    async fn write(&mut self, index: String, data: &[u8]) -> Result<(), Self::Error>;

    /// Read a sequence of bytes at an offset from the backend.
    async fn read(&mut self, index: String) -> Result<Vec<u8>, Self::Error>;
}

#[async_trait]
impl<T: IndexAccess + Send> IndexAccess for Box<T> {
    type Error = T::Error;

    async fn write(&mut self, index: String, data: &[u8]) -> Result<(), Self::Error> {
        self.write(index, data).await
    }

    async fn read(&mut self, index: String) -> Result<Vec<u8>, Self::Error> {
        self.read(index).await
    }
}
