use std::io::{Cursor, Read};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn test() {
    let packet_len = 5;
    let mut value = &mut Cursor::new(vec![1u8, 2u8, 3u8, 4u8, 5u8]);
    let mut buf = &mut Vec::<u8>::new();
    let data = &value.get_ref()[(value.position() as usize)..];
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("test", |b| b.iter(|| test()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
