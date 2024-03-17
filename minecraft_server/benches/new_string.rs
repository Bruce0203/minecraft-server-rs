use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("new string", |b| {
        b.iter(|| {
            for i in 0..10000 {
                let variable = String::new();
                consume(variable);
            }
        });
    });
}

#[no_mangle]
fn consume<T>(t: T) {}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
