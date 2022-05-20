use anyhow::Result;
use std::env;
use index_access_s3::IndexAccessS3;
use index_access_storage::IndexAccess;

#[tokio::main]
pub async fn main() -> Result<()> {
    let root = "/ias3-test";
    let endpoint = env::var("S3_ENDPOINT")?;
    let region = env::var("S3_REGION")?;
    let bucket = env::var("S3_BUCKET")?;
    let access_key = env::var("S3_ACCESS_KEY")?;
    let secret_key = env::var("S3_SECRET_KEY")?;

    let mut ias3 = IndexAccessS3::new(
        &root, &bucket, &region, &endpoint, &access_key, &secret_key
        ).await.unwrap();

    ias3.write("hello".to_owned(), b"world").await.unwrap();
    let data = ias3.read("hello".to_owned()).await.unwrap();
    println!("hello : {}", String::from_utf8_lossy(&data));

    Ok(())
}
