use std::u8;
use std::collections::HashMap;
use rand::Rng;
use tempfile::Builder;
use quickcheck::{Arbitrary, Gen};
use quickcheck_async;

use index_access_storage::IndexAccess;
use index_access_fs::IndexAccessFs;

const MAX_SIZE: u64 = 10;

#[derive(Clone, Debug)]
enum Op {
    Read { index: String },
    Write { index: String, data: Vec<u8> },
}

impl Arbitrary for Op {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let index: String = u32::arbitrary(g).to_string();
        let length: u64 = g.gen_range(0, MAX_SIZE);

        if g.gen::<bool>() {
            Op::Read { index }
        } else {
            let mut data = Vec::with_capacity(length as usize);
            for _ in 0..length {
                data.push(u8::arbitrary(g));
            }
            Op::Write { index, data }
        }
    }
}

#[quickcheck_async::tokio]
async fn implementation_matches_model(ops: Vec<Op>) -> bool {
    let dir = Builder::new()
        .prefix("test_model_iaf").tempdir().unwrap();
    let mut implementation =
        IndexAccessFs::new(dir.path()).await.unwrap();
    let mut model = HashMap::<String, Vec<u8>>::new();

    for op in ops {
        match op {
            Op::Read { index } => {
                match implementation.read(index.clone()).await {
                    Ok(data) => assert_eq!(
                        data,
                        *model.get(&index).unwrap()),
                    Err(_) => assert_eq!(None, model.get(&index)),
                }
            },
            Op::Write { index, data } => {
                implementation.write(index.clone(), &data).await
                    .expect("Writes should be successful.");
                let _ = model.insert(index, data);
            },
        }
    }
    true
}
