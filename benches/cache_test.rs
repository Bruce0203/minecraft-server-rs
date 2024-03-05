use std::{
    io::{Cursor, Read},
    num::NonZeroUsize,
};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use server_workspace::io::fast_map::FastMap;

struct MyStruct {
    value: i32,
}

fn test(map: &mut FastMap<MyStruct>) {
    map.add(|i| Ok(MyStruct { value: 12341 }));
    map.remove(0);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut map = &mut FastMap::<MyStruct>::new();
    c.bench_function("test", |b| b.iter(|| test(map)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
