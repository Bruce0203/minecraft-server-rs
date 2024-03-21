use std::{
    io::{Cursor, Error, ErrorKind, Result},
    ops::DerefMut,
    time::{Duration, SystemTime},
};

use mio::{
    net::{TcpListener, TcpStream},
    Events, Interest, Poll, Token,
};

use crate::io::fast_map::FastMap;

use super::prelude::{Server, Socket};

pub trait Selector {
    fn run<const MAX_PACKET_BUFFER_SIZE: usize>(&mut self);
}

const SERVER_TOKEN_INDEX: usize = usize::MAX;

impl<Server: super::prelude::Server> Selector for Server {
    fn run<const MAX_PACKET_BUFFER_SIZE: usize>(&mut self) {
        let mut poll = Poll::new().unwrap();
        let mut events = Events::with_capacity(128);
        let mut connection_pool = FastMap::<Socket<Server::Player>>::with_capacity(128);

        let addr = "0.0.0.0:25565".parse().unwrap();
        let mut listener = TcpListener::bind(addr).unwrap();
        let server_token = Token(SERVER_TOKEN_INDEX);

        poll.registry()
            .register(&mut listener, server_token, Interest::READABLE)
            .unwrap();

        start_loop_with_listener::<Server, MAX_PACKET_BUFFER_SIZE>(
            self,
            poll,
            events,
            connection_pool,
            listener,
            server_token,
        );
    }
}

fn start_loop_with_listener<S: Server, const MAX_PACKET_BUFFER_SIZE: usize>(
    server: &mut S,
    mut poll: Poll,
    mut events: Events,
    mut connection_pool: FastMap<Socket<S::Player>>,
    listener: TcpListener,
    server_token: Token,
) {
    loop {
        poll.poll(&mut events, Some(Duration::ZERO)).unwrap();

        for event in events.iter() {
            let event_token = event.token();
            let token_index = event_token.0;

            if token_index != SERVER_TOKEN_INDEX {
                let player = connection_pool.get(token_index);

                if let Err(err) = player.handle_read_event::<MAX_PACKET_BUFFER_SIZE, _>(server) {
                    if err.kind() == ErrorKind::BrokenPipe {
                        println!("conneciton closed[{}]: {}", err.kind(), err);
                        connection_pool.remove(token_index);
                    }
                }
            } else {
                let _ = connection_pool.add(|index| match listener.accept() {
                    Ok((mut stream, addr)) => {
                        poll.registry()
                            .register(&mut stream, Token(index), Interest::READABLE)?;
                        let player = Socket::new::<MAX_PACKET_BUFFER_SIZE>(
                            stream,
                            event_token,
                            addr,
                            S::Player::default(),
                        );
                        Ok(player)
                    }
                    Err(err) => Err(err),
                });
            }
        }
    }
}
