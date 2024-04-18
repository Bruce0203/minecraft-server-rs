use std::{collections::VecDeque, io::Cursor};

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("slice to primitive", |b| {
        b.iter(|| i32::from_be_bytes([0, 1, 2, 3]));
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
