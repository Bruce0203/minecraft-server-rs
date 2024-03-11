use minecraft_server::{
    net::prelude::{PacketId, SessionRelay, Socket},
    protocol::v1_20_4::handshake::HandShake,
    server::prelude::GamePlayer,
};
use mio::{
    net::{TcpListener, TcpStream},
    Token,
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
        let player: Socket<GamePlayer> = Socket::new::<100>(
            TcpStream::from_std(
                std::net::TcpStream::connect(listener.local_addr().unwrap()).unwrap(),
            ),
            Token(0),
            listener.local_addr().unwrap(),
            GamePlayer::default(),
        );
        let (stream, addr) = listener.accept().unwrap();
        assert_eq!(addr, player.addr);
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
