use server_workspace::io::prelude::VarIntRead;
use std::{collections::VecDeque, io::Cursor};

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let batch_size = BatchSize::LargeInput;
    c.bench_function("push vec", |b| {
        b.iter_batched(
            || Vec::from([1, 2, 3, 4, 5]),
            |mut vec| {
                vec.push(0);
            },
            batch_size,
        )
    });

    c.bench_function("push vec_deque front", |b| {
        b.iter_batched(
            || VecDeque::<u8>::from([1, 2, 3, 4, 5]),
            |mut vec| {
                vec.push_front(0);
            },
            batch_size,
        );
    });
    c.bench_function("push vec_deque back", |b| {
        b.iter_batched(
            || VecDeque::<u8>::from([1, 2, 3, 4, 5]),
            |mut vec| {
                vec.push_back(0);
            },
            batch_size,
        );
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
