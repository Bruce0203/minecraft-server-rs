use std::rc::Weak;

use criterion::{criterion_group, criterion_main, BatchSize::PerIteration, Criterion};
use minecraft_server::server::prelude::{Entity, EntityMetadata, EntityMetadataValue};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("dyn cost", |b| {
        b.iter(|| {
            let mut meta = Entity::default();
            let value = meta.get_or_else(0, || EntityMetadataValue::Byte(0));
        });
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
