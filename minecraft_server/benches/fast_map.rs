use minecraft_server::io::fast_map::FastMap;
use std::{collections::VecDeque, io::Cursor};

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let batch_size = BatchSize::LargeInput;

    let mut connection_pool = FastMap::<String>::with_capacity(128);
    c.bench_function("fast map", |b| {
        b.iter(|| {
            connection_pool.add(|i| Ok("".to_string())).unwrap();
            connection_pool.remove(0);
        });
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
