use std::{
    io::{Cursor, Error, ErrorKind, Result},
    time::{Duration, SystemTime},
};

use mio::{net::TcpListener, Events, Interest, Poll, Token};

use crate::{
    io::fast_map::FastMap,
    protocol::{
        prelude::{PacketReadHandler, SessionRelay},
        v1_20_4::V1_20_4,
    },
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
    pub const MAX_PACKET_BUFFER_SIZE: u64 = 100_000;

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

        let addr = "0.0.0.0:25565".parse().unwrap();
        let mut listener = TcpListener::bind(addr).unwrap();
        const SERVER_TOKEN_INDEX: usize = usize::MAX;
        let server_token = Token(SERVER_TOKEN_INDEX);

        poll.registry()
            .register(&mut listener, server_token, Interest::READABLE)
            .unwrap();

        loop {
            poll.poll(&mut events, Some(Duration::ZERO)).unwrap();
            let start_time = SystemTime::now();
            let has_events = !events.is_empty();
            for event in events.iter() {
                let event_token = event.token();
                let token_index = event_token.0;

                if token_index != SERVER_TOKEN_INDEX {
                    let player = connection_pool.get(token_index);

                    if let Err(err) = player.handle_packet_read(Self::handle_packet) {
                        if err.kind() == ErrorKind::BrokenPipe {
                            println!("Read error: {}", err);
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
                                read_buf: Cursor::new(Vec::from(
                                    [0; Self::MAX_PACKET_BUFFER_SIZE as usize],
                                )),
                                write_buf: Cursor::new(Vec::from(
                                    [0; Self::MAX_PACKET_BUFFER_SIZE as usize],
                                )),
                                packet_buf: Cursor::new(vec![]),
                            };
                            Ok(player)
                        }
                        Err(err) => Err(err),
                    });
                }
            }
            let end = SystemTime::now();
            if has_events {
                println!("{:?}", end.duration_since(start_time));
            }
        }
    }

    fn handle_packet(player: &mut Player) -> Result<()> {
        println!("asdfasd");
        match player.session_relay.protocol_id {
            0 => {
                V1_20_4::handle_packet_read(player)?;
            }
            765 => {
                V1_20_4::handle_packet_read(player)?;
            }
            n => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("unknown protocol: {:?}", n),
                ))
            }
        }
        Ok(())
    }
}
