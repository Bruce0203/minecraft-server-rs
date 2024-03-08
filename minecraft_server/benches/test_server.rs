use mio::{
    net::{TcpListener, TcpStream},
    Token,
};
use server_workspace::{
    io::prelude::VarIntRead,
    net::prelude::{Player, Selector, SessionRelay, Socket},
    server::prelude::Server,
};
use std::{
    collections::VecDeque,
    fs::File,
    io::Cursor,
    os::{
        fd::{AsRawFd, FromRawFd, RawFd},
        macos::raw,
    },
};

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("test server", |b| {
        let listener = TcpListener::bind("0.0.0.0:0".parse().unwrap()).unwrap();
        let player = Socket {
            stream: TcpStream::from_std(
                std::net::TcpStream::connect(listener.local_addr().unwrap()).unwrap(),
            ),
            token: Token(0),
            addr: listener.local_addr().unwrap(),
            session_relay: SessionRelay::default(),
            read_buf: Cursor::new(Vec::from([0; 10_000])),
            write_buf: Cursor::new(Vec::from([0; 10_000])),
            packet_buf: Cursor::new(vec![]),
        };
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
