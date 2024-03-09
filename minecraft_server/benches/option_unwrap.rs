use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let batch_size = BatchSize::LargeInput;

    c.bench_function("if", |b| {
        b.iter_batched(
            || Box::new(Some("")),
            |mut v| {
                for i in 0..10 {
                    v.unwrap();
                }
            },
            BatchSize::PerIteration,
        );
    });
    c.bench_function("no if ", |b| {
        b.iter_batched(|| {}, |mut v| for i in 0..10 {}, BatchSize::PerIteration);
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
