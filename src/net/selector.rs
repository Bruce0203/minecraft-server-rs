use std::{
    io::{Cursor, Error, ErrorKind, Result},
    time::{Duration, SystemTime},
};

use mio::{net::TcpListener, Events, Interest, Poll, Token};

use crate::{
    io::fast_map::FastMap, net::prelude::{Player, SessionRelay}, server::prelude::Server,
};

use super::prelude::PacketHandler;

#[derive(derive_more::Deref, derive_more::DerefMut)]
pub struct Selector {
    pub server: Server,
}

impl Selector {
    pub fn run<const MAX_PACKET_BUFFER_SIZE: usize>(&mut self) {
        let mut poll = Poll::new().unwrap();
        let mut events = Events::with_capacity(128);
        let mut connection_pool = FastMap::<Player>::with_capacity(128);

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

                    if let Err(err) =
                        player.handle_packet_read::<MAX_PACKET_BUFFER_SIZE>(&mut *self)
                    {
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
                            let player = Player {
                                stream,
                                token: event_token,
                                addr,
                                session_relay: SessionRelay::default(),
                                read_buf: Cursor::new(Vec::from([0; MAX_PACKET_BUFFER_SIZE])),
                                write_buf: Cursor::new(Vec::from([0; MAX_PACKET_BUFFER_SIZE])),
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
