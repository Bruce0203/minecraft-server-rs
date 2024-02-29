use std::net::SocketAddr;

use bytes::BytesMut;
use common_server::selector::Socket;
use mio::{net::TcpStream, Token};

use crate::{connection::session_relay::SessionRelay, server::Server};

pub struct Player {
    pub stream: TcpStream,
    pub token: Token,
    pub addr: SocketAddr,
    pub session_relay: SessionRelay,
    pub write_buffer: BytesMut,
}

impl Socket for Player {
    type Server = Server;
    fn stream(&mut self) -> &mut TcpStream {
        &mut self.stream
    }

    fn token(&self) -> Token {
        self.token
    }

    fn addr(&self) -> SocketAddr {
        self.addr
    }

    fn get_wirte_buffer(&mut self) -> &mut BytesMut {
        &mut self.write_buffer
    }
}
