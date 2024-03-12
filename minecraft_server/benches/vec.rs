use std::{collections::VecDeque, io::Cursor};

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let batch_size = BatchSize::LargeInput;
    let vec = vec![1; 1000];
    c.bench_function("push vec", |b| {
        b.iter(|| {
            let mut i = 0;
            for ele in vec.iter() {
                i += 1;
                let _ = ele + 1;
            }
            assert_eq!(i, vec.len());
        });
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
