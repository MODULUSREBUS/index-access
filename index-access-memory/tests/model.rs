use quickcheck::{Arbitrary, Gen};
use quickcheck_async;
use std::collections::HashMap;
use std::u8;

use index_access_memory::IndexAccessMemory;
use index_access_storage::IndexAccess;

#[derive(Clone, Debug)]
enum Op {
    Read { index: u32 },
    Write { index: u32, data: Vec<u8> },
}

impl Arbitrary for Op {
    fn arbitrary(g: &mut Gen) -> Self {
        let index = u8::arbitrary(g) as u32;
        let lengths = [0, 1, 2, 50];
        let length: u64 = *g.choose(&lengths).unwrap();

        if bool::arbitrary(g) {
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
    let mut implementation = IndexAccessMemory::default();
    let mut model = HashMap::<u32, Vec<u8>>::new();

    for op in ops {
        match op {
            Op::Read { index } => match implementation.read(index.clone()).await {
                Ok(Some(data)) => assert_eq!(data, *model.get(&index).unwrap()),
                Ok(None) => assert_eq!(None, model.get(&index)),
                Err(_) => return false,
            },
            Op::Write { index, data } => {
                implementation
                    .write(index.clone(), &data)
                    .await
                    .expect("write to index_access");
                let _ = model.insert(index, data);
            }
        }
    }
    true
}
