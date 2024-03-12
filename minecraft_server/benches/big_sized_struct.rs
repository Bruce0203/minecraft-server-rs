use std::{collections::VecDeque, io::Cursor};

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

struct MyStruct {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    e: i32,
   k: i32,
    l: i32,
    m: i32,
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("push vec", |b| {
        b.iter(|| MyStruct {
            a: 1,
            b: 1,
            c: 1,
            d: 1,
            e: 1,
            k: 1,
            l: 1,
            m: 1,
        });
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
