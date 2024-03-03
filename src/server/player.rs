use std::net::SocketAddr;

use mio::{net::TcpStream, Token};

use crate::protocol::prelude::SessionRelay;

use super::server::Server;

pub struct Player {
    pub stream: TcpStream,
    pub token: Token,
    pub addr: SocketAddr,
    pub session_relay: SessionRelay,
}

impl Player {
    fn handle_connection_closed(&mut self) {}
}
