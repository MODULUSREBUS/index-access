use futures::future;
use hyper::server::Server;
use hyper::service::make_service_fn;
use s3_server::storages::fs::FileSystem;
use s3_server::S3Service;
use s3_server::SimpleAuth;
use std::mem::drop;
use std::net::TcpListener;
use tempfile::Builder;
use tokio::{task, test};

use index_access_s3::IndexAccessS3;
use index_access_storage::IndexAccess;

#[test]
async fn can_create_new() {
    // server
    let dir = Builder::new().prefix("test_ias3_write").tempdir().unwrap();

    let fs = FileSystem::new(&dir).unwrap();
    let service = S3Service::new(fs);

    let server = {
        let service = service.into_shared();
        let listener = TcpListener::bind("localhost:9018").unwrap();
        let make_service: _ =
            make_service_fn(move |_| future::ready(Ok::<_, anyhow::Error>(service.clone())));
        Server::from_tcp(listener).unwrap().serve(make_service)
    };
    let server = task::spawn(server);

    let _ias3 = IndexAccessS3::new("/", "bucket", "nyc", "http://127.0.0.1:9017", "", "").unwrap();
    drop(server);
}

#[test]
async fn can_write() {
    let access_key = "access".to_owned();
    let secret_key = "secret".to_owned();

    // server
    let dir = Builder::new().prefix("test_ias3_write").tempdir().unwrap();

    let fs = FileSystem::new(&dir).unwrap();
    let mut service = S3Service::new(fs);
    let mut auth = SimpleAuth::new();
    auth.register(access_key.clone(), secret_key.clone());
    service.set_auth(auth);

    let server = {
        let service = service.into_shared();
        let listener = TcpListener::bind("localhost:9018").unwrap();
        let make_service: _ =
            make_service_fn(move |_| future::ready(Ok::<_, anyhow::Error>(service.clone())));
        Server::from_tcp(listener).unwrap().serve(make_service)
    };
    let server = task::spawn(server);

    // client
    let mut ias3 = IndexAccessS3::new(
        "/",
        "bucket",
        "nyc",
        "http://localhost:9018",
        &access_key,
        &secret_key,
    )
    .unwrap();

    // run
    ias3.write(0, b"hello world").await.unwrap();
    drop(server);
}

#[test]
async fn can_read() {
    let access_key = "access".to_owned();
    let secret_key = "secret".to_owned();
    let data = b"some data string to be written and read back";

    // server
    let dir = Builder::new().prefix("test_ias3_read").tempdir().unwrap();

    let fs = FileSystem::new(&dir).unwrap();
    let mut service = S3Service::new(fs);
    let mut auth = SimpleAuth::new();
    auth.register(access_key.clone(), secret_key.clone());
    service.set_auth(auth);

    let server = {
        let service = service.into_shared();
        let listener = TcpListener::bind("localhost:9019").unwrap();
        let make_service: _ =
            make_service_fn(move |_| future::ready(Ok::<_, anyhow::Error>(service.clone())));
        Server::from_tcp(listener).unwrap().serve(make_service)
    };
    let server = task::spawn(server);

    // client
    let mut ias3 = IndexAccessS3::new(
        "/",
        "bucket",
        "nyc",
        "http://localhost:9019",
        &access_key,
        &secret_key,
    )
    .unwrap();

    // run
    let index: u32 = 100000;
    ias3.write(index, data).await.unwrap();
    let read = ias3.read(index).await.unwrap();
    assert_eq!(read, data);
    drop(server);
}
