use bytes::{BufMut, BytesMut};
use mio::{
    net::{TcpListener, TcpStream},
    Events, Interest, Poll, Token,
};
use std::{
    io::{Read, Result, Write},
    net::SocketAddr,
    time::Duration,
};

use crate::var_int::VarIntWrite;

pub trait Socket: Sized {
    type Server: ConnectionHandler<Self>;

    fn stream(&mut self) -> &mut TcpStream;

    fn token(&self) -> Token;

    fn addr(&self) -> SocketAddr;

    fn get_write_buf(&mut self) -> &mut BytesMut;
}

pub struct ConnectionPool<Player: Socket> {
    pub indexed_connection: Vec<Option<Player>>,
    pub index_queue: Vec<usize>,
}

impl<Player: Socket> ConnectionPool<Player> {
    fn get_socket(&mut self, token_index: usize) -> &mut Player {
        unsafe { self.indexed_connection.get_unchecked_mut(token_index) }
            .as_mut()
            .expect("socket is none")
    }
}

pub trait ConnectionHandler<Player: Socket>: Sized {
    fn handle_connection_accept(
        &mut self,
        stream: TcpStream,
        token: Token,
        addr: SocketAddr,
    ) -> Player;

    fn handle_connection_read(&mut self, socket: &mut Player, buf: &[u8]) -> Result<()>;

    fn handle_connection_closed(&mut self, socket: &mut Player);

    fn handle_update(&mut self);
}

pub struct Selector<Player: Socket, Server: ConnectionHandler<Player>> {
    listener: TcpListener,
    poll: Poll,
    connection_pool: ConnectionPool<Player>,
    connection_handler: Box<Server>,
}

impl<Player: Socket, Server: ConnectionHandler<Player>> Selector<Player, Server> {
    pub fn new(
        addr: SocketAddr,
        connection_pool_size: usize,
        connection_handler: Server,
    ) -> Selector<Player, Server> {
        Selector {
            listener: TcpListener::bind(addr).expect("Cannot start server"),
            poll: Poll::new().expect("cannot create poll"),
            connection_pool: ConnectionPool {
                indexed_connection: Vec::with_capacity(connection_pool_size),
                index_queue: Vec::with_capacity(connection_pool_size),
            },
            connection_handler: Box::new(connection_handler),
        }
    }

    pub fn start_selection_loop(mut self, timeout: Option<Duration>) {
        let server_token = Token(usize::MAX);
        let poll = &mut self.poll;
        let listener = &mut self.listener;
        let connection_handler = &mut self.connection_handler;
        let connection_pool = &mut self.connection_pool;
        poll.registry()
            .register(listener, server_token, Interest::READABLE)
            .expect("Cannot reigster server to poll");
        const MAX_READ_BUFFER_SIZE: usize = 10000;
        let buf = &mut [0u8; MAX_READ_BUFFER_SIZE];
        let events_capacity = 128;
        let events = &mut Events::with_capacity(events_capacity);
        loop {
            #[warn(unused_must_use)]
            if let Err(_) = poll.poll(events, timeout) {
                continue;
            }
            connection_handler.handle_update();
            for event in events.iter() {
                let token = event.token();
                if token == server_token {
                    if let Ok((stream, addr)) = listener.accept() {
                        if let Some(index) = connection_pool.index_queue.pop() {
                            let token = Token(index);
                            let mut connection =
                                connection_handler.handle_connection_accept(stream, token, addr);
                            poll.registry()
                                .register(connection.stream(), Token(index), Interest::READABLE)
                                .expect("poll register");
                            connection_pool.indexed_connection[index] = Some(connection);
                        } else {
                            let index = connection_pool.indexed_connection.len();
                            let token = Token(index);
                            let mut connection =
                                connection_handler.handle_connection_accept(stream, token, addr);
                            poll.registry()
                                .register(connection.stream(), Token(index), Interest::READABLE)
                                .expect("poll register");
                            connection_pool.indexed_connection.push(Some(connection));
                        }
                    }
                } else {
                    let token_index = token.0;
                    if event.is_readable() {
                        let player = connection_pool.get_socket(token_index);
                        let stream = player.stream();
                        let read_result = stream.read(buf);
                        if read_result.is_err() {
                            poll.registry()
                                .deregister(player.stream())
                                .expect("cannot deregister socket");
                            connection_handler.handle_connection_closed(player);
                            connection_pool.index_queue.push(token_index);
                            connection_pool.indexed_connection[token_index] = None;
                            continue;
                        }
                        let read = read_result.unwrap();
                        if read == 0 {
                            poll.registry()
                                .deregister(player.stream())
                                .expect("cannot deregister socket");
                            connection_handler.handle_connection_closed(player);
                            connection_pool.index_queue.push(token_index);
                            connection_pool.indexed_connection[token_index] = None;
                            continue;
                        } else {
                            let read_buf = &buf[0..read];
                            if let Err(err) =
                                connection_handler.handle_connection_read(player, read_buf)
                            {
                                println!("Read handle error: {}", err);
                                poll.registry()
                                    .deregister(player.stream())
                                    .expect("cannot deregister socket");
                                connection_handler.handle_connection_closed(player);
                                connection_pool.index_queue.push(token_index);
                                connection_pool.indexed_connection[token_index] = None;
                                continue;
                            }
                            if !player.get_write_buf().is_empty() {
                                poll.registry()
                                    .reregister(
                                        player.stream(),
                                        token,
                                        Interest::READABLE | Interest::WRITABLE,
                                    )
                                    .expect("cannot reregister socket");
                            }
                        }
                    } else if event.is_writable() {
                        let player = connection_pool.get_socket(token_index);
                        let mut write_buf = player.get_write_buf().clone();
                        let stream = player.stream();
                        poll.registry()
                            .reregister(stream, token, Interest::READABLE)
                            .expect("cannot reregister socket");
                        if let Err(_) = stream.write_all(&write_buf) {
                            poll.registry()
                                .deregister(player.stream())
                                .expect("cannot deregister socket");
                            connection_handler.handle_connection_closed(player);
                            connection_pool.index_queue.push(token_index);
                            connection_pool.indexed_connection[token_index] = None;
                            continue;
                        }
                        write_buf.clear();
                    }
                }
            }
        }
    }
}
