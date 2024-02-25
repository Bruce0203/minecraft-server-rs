use std::io::{Error, ErrorKind, Result, Write};

use bytes::{Buf, BytesMut};
use common_server::{
    selector::{ConnectionHandler, Selector, Socket},
    var_int::VarIntRead,
};

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
    let mut reader = BytesMut::from(buf).reader();
    let packet_length = reader.read_var_i32();
    let bytes = reader.into_inner();
    if bytes.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!(
                "packet length is {:?}, but buffer size is {:?}",
                packet_length,
                bytes.len()
            ),
        ));
    }
    packet::handle_packet(socket, bytes)?;
    Ok(())
}

#[test]
fn test_handshake_server() {
    let mut selector = Selector::bind("127.0.0.1:25432".parse().unwrap(), 256);
    let mut server = Server::new();
    selector.start_selection_loop(&mut server);
}
