use std::time::{Duration, SystemTime};

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("match i32", |b| {
        b.iter_batched(
            || (),
            |r| {
                for _ in 0..i32::MAX {
                    match asdf() {
                        1 => {}
                        2 => {}
                        3 => {}
                        4 => {}
                        5 => {}
                        5 => {}
                        6 => {}
                        7 => {}
                        8 => {}
                        9 => {}
                        10 => {}
                        11 => {}
                        12 => {}
                        13 => {}
                        14 => {}
                        15 => {}
                        16 => {}
                        n => {}
                    }
                }
            },
            BatchSize::PerIteration,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

#[no_mangle]
fn asdf() -> i32 {
    14
}
