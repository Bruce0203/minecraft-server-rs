use std::{
    io::{Cursor, Write},
    net::SocketAddr,
};

use mio::{net::TcpStream, Token};
use socket_selector::Socket;

use crate::{connection::session_relay::SessionRelay, server::Server};

pub struct Player {
    pub stream: TcpStream,
    pub token: Token,
    pub addr: SocketAddr,
    pub session_relay: SessionRelay,
}

impl Socket for Player {
    type Server = Server;
    fn get_stream(&mut self) -> &mut TcpStream {
        &mut self.stream
    }

    fn get_token(&self) -> &Token {
        &self.token
    }

    fn get_addr(&self) -> &std::net::SocketAddr {
        &self.addr
    }
}
