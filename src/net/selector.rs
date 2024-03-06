use std::{
    io::{Cursor, Error, ErrorKind, Result},
    time::{Duration, SystemTime},
};

use mio::{net::TcpListener, Events, Interest, Poll, Token};

use crate::{
    io::fast_map::FastMap,
    protocol::{
        prelude::{PacketHandler, SessionRelay},
        v1_20_4::V1_20_4,
    },
    server::prelude::{Player, Server},
};

pub struct Selector {
    pub server: Server,
}

impl Selector {
    pub const MAX_PACKET_BUFFER_SIZE: u64 = 100;

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
            for event in events.iter() {
                let event_token = event.token();
                let token_index = event_token.0;

                if token_index != SERVER_TOKEN_INDEX {
                    let player = connection_pool.get(token_index);

                    let server = &mut self.server;
                    if let Err(err) =
                        player.handle_packet_read(move |player| Self::handle_packet(server, player))
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
                                read_buf: Cursor::new(Vec::from([0; 500])),
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
        }
    }

    fn handle_packet(server: &mut Server, player: &mut Player) -> Result<()> {
        match player.session_relay.protocol_id {
            0 => {
                PacketHandler::handle_packet(&V1_20_4, server, player)?;
            }
            765 => {
                PacketHandler::handle_packet(&V1_20_4, server, player)?;
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
