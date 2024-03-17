use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    struct MyStruct {
        foo: i32,
        bar: i32,
    }
    let struc = MyStruct {
        foo: 1234,
        bar: 5678,
    };
    let vec = vec!["a", "b", "c"];
    c.bench_function("query vec", |b| {
        b.iter(|| {
            for i in 0..10000 {
                let _ = vec[1];
            }
        });
    });
    c.bench_function("query struct", |b| {
        b.iter(|| {
            for i in 0..10000 {
                consume(struc.bar)
            }
        });
    });
}

#[no_mangle]
fn consume<T>(t: T) {}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
