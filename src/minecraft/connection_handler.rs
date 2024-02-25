use std::io::{Error, ErrorKind, Result, Write};

use bytes::{Buf, BytesMut};

use crate::{
    selector::{ConnectionHandler, Socket},
    var_int::VarIntRead,
    var_string::VarStringRead,
};

use super::player::Player;

struct PreJoinHandler {}

impl ConnectionHandler<Player> for PreJoinHandler {
    fn handle_connection_accept(&mut self) -> Player {
        Player::default()
    }

    fn handle_connection_read(&mut self, socket: &mut Socket<Player>, buf: &[u8]) {
        socket.handle_packet_read(buf);
    }

    fn handle_connection_closed(&mut self, _socket: &mut Socket<Player>) {}
}

impl Socket<Player> {
    fn handle_packet_read(&mut self, buf: &[u8]) {
        let mut reader = BytesMut::from(buf).reader();
        let packet_length = reader.read_var_i32();
        let session_relay = &mut self.connection.session_relay;
        if let Err(_) = &self.handle_packet(reader.into_inner()) {
            self.stream.flush();
        }
    }
}

#[test]
fn test_handshake_server() {
    println!("Server started!");
    let mut server = PreJoinHandler {};
    server.start_selection_loop("127.0.0.1:25565".parse().unwrap(), 100);
}
