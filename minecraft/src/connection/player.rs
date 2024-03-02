use std::{net::SocketAddr, io::{Write, Cursor}};

use common_server::selector::Socket;
use mio::{net::TcpStream, Token};

use crate::{connection::session_relay::SessionRelay, server::Server};

pub struct Player {
    pub stream: TcpStream,
    pub token: Token,
    pub addr: SocketAddr,
    pub session_relay: SessionRelay,
    pub write_buffer: Cursor<Vec<u8>>,
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

    fn get_write_buf(&mut self) -> &mut Cursor<Vec<u8>> {
        &mut self.write_buffer
    }

    fn write_buf_to_stream(&mut self) -> std::io::Result<()> {
        self.stream.write_all(self.write_buffer.get_ref())?;
        Ok(())
    }
}
