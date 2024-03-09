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

    c.bench_function("if", |b| {
        b.iter(|| {
            for i in 0..10 {
                if true {}
            }
        });
    });
    c.bench_function("no if ", |b| {
        b.iter(|| for i in 0..10 {});
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
