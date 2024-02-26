use std::io::{Result, Write};

use bytes::{Buf, BytesMut};
use common_server::selector::{ConnectionHandler, Socket};

use crate::packet;

use super::{player::Player, server::Server};

impl ConnectionHandler<Player> for Server {
    fn handle_connection_accept(&mut self) -> Player {
        Player::default()
    }

    fn handle_connection_read(&mut self, socket: &mut Socket<Player>, buf: &[u8]) {
        if let Err(_) = handle_packet_read(socket, buf) {
            socket.stream.flush().unwrap();
        }
    }

    fn handle_connection_closed(&mut self, _socket: &mut Socket<Player>) {}
}

fn handle_packet_read(socket: &mut Socket<Player>, buf: &[u8]) -> Result<()> {
    let reader = BytesMut::from(buf).reader();
    let bytes = reader.into_inner();
    packet::handle_packet(socket, bytes)?;
    Ok(())
}
