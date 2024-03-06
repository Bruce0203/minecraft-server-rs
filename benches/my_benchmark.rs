use std::io::{Cursor, Read};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use server_workspace::{io::var_int::VarIntRead, net::prelude::Selector, server::server::Server};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("test", |b| b.iter(|| {
        std::thread::spawn(|| {
            Selector {
                server: Server::new();
            }.run();
        });
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
