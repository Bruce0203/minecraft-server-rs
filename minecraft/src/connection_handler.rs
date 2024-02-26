use std::io::{Error, ErrorKind, Result, Write};

use bytes::{Buf, BufMut, BytesMut};
use common_server::{
    encoder::Encoder,
    selector::{ConnectionHandler, Selector, Socket},
    var_int::{VarIntRead, VarIntWrite},
};

use crate::protocol::v1_20_4::V1_20_4;

use super::{packet_read_handler::PacketReadHandler, player::Player, server::Server};

impl ConnectionHandler<Player> for Server {
    fn handle_connection_accept(&mut self) -> Player {
        Player::default()
    }

    fn handle_connection_read(&mut self, socket: &mut Socket<Player>, buf: &[u8]) {
        if let Err(_) = self.handle_packet_read(socket, buf) {
            socket.stream.flush().unwrap();
        }
    }

    fn handle_connection_closed(&mut self, _socket: &mut Socket<Player>) {}
}

impl Server {
    fn handle_packet_read(&mut self, socket: &mut Socket<Player>, buf: &[u8]) -> Result<()> {
        let mut reader = BytesMut::from(buf).reader();
        let packet_length = reader.read_var_i32();
        let value = reader.into_inner();
        if value.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "packet length is {:?}, but buffer size is {:?}",
                    packet_length,
                    value.len()
                ),
            ));
        }
        match socket.connection.session_relay.protocol_id {
            0 =>  {
                V1_20_4::handle_packet_read(self, socket, value)?;
            }
            765 => {
                V1_20_4::handle_packet_read(self, socket, value)?;
            }
            n => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("unknown protocol: {:?}", n),
                ))
            }
        }
        Ok(())
    }
}

#[test]
fn test_handshake_server() {
    println!("Server started!");
    let mut server = Server::new();
    let mut selector = Selector::bind("127.0.0.1:25565".parse().unwrap(), 256);
    selector.start_selection_loop(&mut server);
}
