use std::time::Duration;

use fast_id_map::prelude::FastMap;
use mio::{net::TcpListener, Events, Interest, Poll, Token};

use crate::{
    protocol::{packet_reader, prelude::SessionRelay},
    server::prelude::Player,
};

use super::{
    chat::Chat,
    server_status::{Players, SamplePlayers, ServerStatus, ServerVersion},
};

pub struct Server {
    pub server_status: ServerStatus,
}

impl Server {
    pub fn new() -> Server {
        Server {
            server_status: ServerStatus {
                version: ServerVersion {
                    name: "1.20.4".to_string(),
                    protocol: 765,
                },
                description: Chat::from("A Minecraft Server".to_string()),
                favicon: None,
                enforce_secure_chat: true,
                previews_chat: true,
                players: Players {
                    max: 20,
                    online: 0,
                    sample: SamplePlayers::new(),
                },
            },
        }
    }

    pub fn run(&mut self) {
        let mut poll = Poll::new().unwrap();
        let mut events = Events::with_capacity(128);
        let mut connection_pool = FastMap::<Player>::with_capacity(128);
        let read_buf = &mut [0; 10000];

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
                    if let Err(err) = packet_reader::read_packet(self, player, read_buf) {
                        println!("Read error: {}", err);
                        connection_pool.remove(token_index);
                    }
                } else {
                    let _ = connection_pool.add(|index| match listener.accept() {
                        Ok((mut stream, addr)) => {
                            poll.registry().register(
                                &mut stream,
                                Token(index),
                                Interest::READABLE,
                            )?;
                            Ok(Player {
                                stream,
                                token: event_token,
                                addr,
                                session_relay: SessionRelay::default(),
                            })
                        }
                        Err(err) => Err(err),
                    });
                }
            }
        }
    }
}
