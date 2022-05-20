# IndexAccess

- index-access-storage
- index-access-memory
- index-access-fs
- index-access-s3

### index-access-storage

Interface describing random access read/write.

### index-access-memory

Implementation of `index-access-storage` for memory.
Useful for testing or ephemeral storage.

### index-access-fs

Implementation of `index-access-storage` for file system storage.
The most basic persistent implementation, no performance optimizations.
All writes are immediately flushed - low throughput, durability.

### index-access-s3

Implementation of `index-access-storage` for S3-compatible APIs.
