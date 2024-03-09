use std::{
    io::{Cursor, Error, ErrorKind, Result},
    ops::DerefMut,
    time::{Duration, SystemTime},
};

use mio::{net::TcpListener, Events, Interest, Poll, Token};

use crate::{
    io::fast_map::FastMap,
    net::{prelude::SessionRelay, socket::Socket},
};

use super::prelude::Server;

pub trait Selector {
    fn run(&mut self);
}

impl<Server: super::prelude::Server> Selector for Server {
    fn run(&mut self) {
        let mut poll = Poll::new().unwrap();
        let mut events = Events::with_capacity(128);
        let mut connection_pool = FastMap::<Socket<Server::Player>>::with_capacity(128);

        let addr = "0.0.0.0:25565".parse().unwrap();
        let mut listener = TcpListener::bind(addr).unwrap();
        const SERVER_TOKEN_INDEX: usize = usize::MAX;
        let server_token = Token(SERVER_TOKEN_INDEX);

        poll.registry()
            .register(&mut listener, server_token, Interest::READABLE)
            .unwrap();

        loop {
            poll.poll(&mut events, Some(Duration::ZERO)).unwrap();

            for event in events.iter() {
                let event_token = event.token();
                let token_index = event_token.0;

                if token_index != SERVER_TOKEN_INDEX {
                    let player = connection_pool.get(token_index);

                    if let Err(err) = self.handle_read_event(player) {
                        if err.kind() == ErrorKind::BrokenPipe {
                            println!("conneciton closed[{}]: {}", err.kind(), err);
                            connection_pool.remove(token_index);
                        }
                    }
                } else {
                    let _ = connection_pool.add(|index| match listener.accept() {
                        Ok((mut stream, addr)) => {
                            poll.registry().register(
                                &mut stream,
                                Token(index),
                                Interest::READABLE,
                            )?;
                            let player = Socket {
                                stream,
                                token: event_token,
                                addr,
                                player_data: Server::Player::default(),
                                session_relay: SessionRelay::default(),
                                read_buf: Cursor::new(Vec::from([0; 1000])),
                                write_buf: Cursor::new(Vec::from([0; 1000])),
                                packet_buf: Cursor::new(vec![]),
                            };
                            Ok(player)
                        }
                        Err(err) => Err(err),
                    });
                }
            }
        }
    }
}
