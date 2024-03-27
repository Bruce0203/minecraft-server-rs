use std::{
    io::{Cursor, Error, ErrorKind, Result},
    net::SocketAddr,
    ops::DerefMut,
    time::{Duration, SystemTime},
};

use mio::{
    net::{TcpListener, TcpStream},
    Events, Interest, Poll, Registry, Token,
};

use crate::{
    io::fast_map::FastMap, protocol::v1_20_4::configuration::registry,
    server::prelude::SessionRelay,
};

use super::prelude::{Server, Socket};

pub struct SocketSelector<S: Server> {
    pub server: S,
    pub poll: Poll,
    pub connection_pool: FastMap<Socket<S::Player>>,
    pub events: Events,
}

impl<S: Server> SocketSelector<S>
where
    [(); { S::MAX_PACKET_BUFFER_SIZE }]:,
{
    pub fn new(server: S) -> SocketSelector<S> {
        SocketSelector {
            server,
            poll: Poll::new().unwrap(),
            connection_pool: FastMap::with_capacity(128),
            events: Events::with_capacity(128),
        }
    }

    pub fn connect_client<F: FnOnce(&mut Socket<S::Player>) -> Result<()>>(
        &mut self,
        addr: SocketAddr,
        f: F,
    ) -> Result<()> {
        self.connection_pool.add(|index| {
            let mut player = Socket::new::<{ S::MAX_PACKET_BUFFER_SIZE }>(
                TcpStream::from_std(std::net::TcpStream::connect(addr)?),
                Token(index),
                addr,
                S::Player::default(),
            );
            self.poll
                .registry()
                .register(&mut player.stream, player.token, Interest::READABLE)?;
            f(&mut player)?;
            Ok(player)
        })
    }
}

pub trait Selector {
    fn run_with_listener(&mut self, addr: SocketAddr);
    fn run(&mut self);
}

pub trait SelectorUpdateListener<S: Server> {
    fn on_update(selector: &mut SocketSelector<S>) {}
    fn on_init(selector: &mut SocketSelector<S>) {}
}

pub trait SelectorTicker {
    fn on_tick(&mut self);
}

const SERVER_TOKEN_INDEX: usize = usize::MAX;

fn start_loop_with_listener<S: Server>(
    selector: &mut SocketSelector<S>,
    listener: TcpListener,
    server_token: Token,
) where
    [(); { S::MAX_PACKET_BUFFER_SIZE }]:,
{
    let poll = &mut selector.poll;
    let events = &mut selector.events;
    let connection_pool = &mut selector.connection_pool;
    let server = &mut selector.server;
    loop {
        poll.poll(events, Some(Duration::ZERO)).unwrap();

        for event in events.iter() {
            let event_token = event.token();
            let token_index = event_token.0;

            if token_index != SERVER_TOKEN_INDEX {
                let player = connection_pool.get(token_index);

                if let Err(err) = player.handle_read_event(server) {
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
                        let player = Socket::new::<{ S::MAX_PACKET_BUFFER_SIZE }>(
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

fn start_loop<S: Server>(socket_selector: &mut SocketSelector<S>)
where
    [(); { S::MAX_PACKET_BUFFER_SIZE }]:,
{
    S::on_init(socket_selector);
    loop {
        S::on_update(socket_selector);
        socket_selector
            .poll
            .poll(&mut socket_selector.events, Some(Duration::ZERO))
            .unwrap();

        for event in socket_selector.events.iter() {
            let event_token = event.token();
            let token_index = event_token.0;

            let player = socket_selector.connection_pool.get(token_index);
            if let Err(err) = player.handle_read_event(&mut socket_selector.server) {
                if err.kind() == ErrorKind::BrokenPipe {
                    println!("err");
                    println!("conneciton closed[{}]: {}", err.kind(), err);
                    player.stream.shutdown(std::net::Shutdown::Both);
                    socket_selector.connection_pool.remove(token_index);
                }
            }
        }
    }
}

impl<S: Server> Selector for SocketSelector<S>
where
    [(); { S::MAX_PACKET_BUFFER_SIZE }]:,
{
    fn run_with_listener(&mut self, addr: SocketAddr) {
        let mut poll = Poll::new().unwrap();
        let mut events = Events::with_capacity(128);

        let mut listener = TcpListener::bind(addr).unwrap();
        let server_token = Token(SERVER_TOKEN_INDEX);
        poll.registry()
            .register(&mut listener, server_token, Interest::READABLE)
            .unwrap();

        start_loop_with_listener::<S>(self, listener, server_token);
    }

    fn run(&mut self) {
        start_loop(self);
    }
}
