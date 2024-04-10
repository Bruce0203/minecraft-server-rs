use std::{fs::File, io::Read};

use criterion::{criterion_group, criterion_main, BatchSize::PerIteration, Criterion};

fn benchmark(c: &mut Criterion) {
    c.bench_function("hi", |b| {
        b.iter_batched_ref(
            || File::open("tests/hello_world.nbt").unwrap(),
            |value| {
                println!("");
            },
            PerIteration,
        );
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
