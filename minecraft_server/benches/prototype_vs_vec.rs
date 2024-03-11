use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let vec = vec!["a", "b", "c"];
    c.bench_function("prototype vs vec", |b| {
        b.iter(|| {
            let _ = vec[1];
        });
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
