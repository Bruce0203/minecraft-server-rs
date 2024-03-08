use std::{io::Cursor, net::SocketAddr, str::FromStr};

use criterion::{criterion_group, criterion_main, BatchSize::PerIteration, Criterion};
use mio::{
    net::{TcpListener, TcpStream},
    Events, Interest, Poll, Token,
};
use server_workspace::{
    net::prelude::{PacketIdentnifier, SessionRelay, Socket},
    protocol::v1_20_4::{
        handshake::{HandShake, NextState::Status},
        login_start::LoginStart,
    },
    server::prelude::Server,
};
use uuid::Uuid;

fn encoding(c: &mut Criterion) {
    let mut listener = TcpListener::bind("0.0.0.0:0".parse().unwrap()).unwrap();
    let mut server = Server::new();
    let mut client = new_client(listener.local_addr().unwrap());
    let (client_stream, client_addr) = listener.accept().unwrap();
    assert_eq!(client_addr, client.addr);
    let poll = Poll::new().unwrap();
    poll.registry()
        .register(&mut listener, client.token, Interest::READABLE)
        .unwrap();
    let events = Events::with_capacity(128);
    for event in events.iter() {
        if event.token() == client.token {
            let server = &mut server;
            c.bench_function("handle packet read", |b| {
                HandShake {
                    protocol_version: server.server_status.version.protocol,
                    server_address: client_addr.to_string(),
                    server_port: client_addr.port(),
                    next_state: Status,
                }
                .send_packet(&mut client)
                .unwrap();
                LoginStart {
                    name: "JetBrainer".to_string(),
                    player_uuid: Uuid::from_str("053d384b-5b9f-47d7-a5da-6885c497ce7f").unwrap(),
                }
                .send_packet(&mut client)
                .unwrap();
                client.handle_packet_read::<10_000>(server).unwrap();
            });
        }
    }
}

fn new_client(addr: SocketAddr) -> Socket {
    Socket {
        stream: TcpStream::from_std(std::net::TcpStream::connect(addr).unwrap()),
        token: Token(0),
        addr,
        session_relay: SessionRelay::default(),
        read_buf: Cursor::new(Vec::from([0; 10_000])),
        write_buf: Cursor::new(Vec::from([0; 10_000])),
        packet_buf: Cursor::new(vec![]),
    }
}

criterion_group!(benches, encoding);
criterion_main!(benches);