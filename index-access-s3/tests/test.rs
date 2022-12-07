use futures::future;
use hyper::server::Server;
use hyper::service::make_service_fn;
use s3_server::storages::fs::FileSystem;
use s3_server::S3Service;
use s3_server::SimpleAuth;
use std::future::Future;
use std::mem::drop;
use std::net::TcpListener;
use tempfile::{Builder, TempDir};
use tokio::task;

use index_access_s3::IndexAccessS3;
use index_access_storage::IndexAccess;

fn create_s3_server(
    dir: TempDir,
    port: u32,
    access_key: &str,
    secret_key: &str,
) -> impl Future<Output = Result<(), hyper::Error>> {
    let fs = FileSystem::new(&dir).unwrap();
    let mut service = S3Service::new(fs);
    let mut auth = SimpleAuth::new();
    auth.register(access_key.to_string(), secret_key.to_string());
    service.set_auth(auth);
    let service = service.into_shared();
    let listener = TcpListener::bind(format!("localhost:{port}")).unwrap();
    let make_service =
        make_service_fn(move |_| future::ready(Ok::<_, anyhow::Error>(service.clone())));
    Server::from_tcp(listener).unwrap().serve(make_service)
}

#[tokio::test]
async fn can_create_new() {
    let access_key = "access".to_owned();
    let secret_key = "secret".to_owned();

    let port = 9801;
    let dir = Builder::new().prefix("test_ias3_new").tempdir().unwrap();
    let server = task::spawn(create_s3_server(dir, port, &access_key, &secret_key));

    let _ias3 = IndexAccessS3::new(
        "/",
        "bucket",
        "nyc",
        &format!("http://localhost:{port}"),
        &access_key,
        &secret_key,
    )
    .unwrap();
    drop(server);
}

#[tokio::test]
async fn can_write() {
    let access_key = "access".to_owned();
    let secret_key = "secret".to_owned();

    let port = 9802;
    let dir = Builder::new().prefix("test_ias3_write").tempdir().unwrap();
    let server = task::spawn(create_s3_server(dir, port, &access_key, &secret_key));

    // client
    let mut ias3 = IndexAccessS3::new(
        "/",
        "bucket",
        "nyc",
        &format!("http://localhost:{port}"),
        &access_key,
        &secret_key,
    )
    .unwrap();

    // run
    ias3.write(0, b"hello world").await.unwrap();
    drop(server);
}

#[tokio::test]
async fn can_read() {
    let access_key = "access".to_owned();
    let secret_key = "secret".to_owned();

    let port = 9803;
    let dir = Builder::new().prefix("test_ias3_read").tempdir().unwrap();
    let server = task::spawn(create_s3_server(dir, port, &access_key, &secret_key));

    // client
    let mut ias3 = IndexAccessS3::new(
        "/",
        "bucket",
        "nyc",
        &format!("http://localhost:{port}"),
        &access_key,
        &secret_key,
    )
    .unwrap();

    // run
    let index: u32 = 100000;
    let data = b"some data string to be written and read back";
    ias3.write(index, data).await.unwrap();
    let read = ias3.read(index).await.unwrap();
    assert_eq!(read.unwrap(), data);
    drop(server);
}
