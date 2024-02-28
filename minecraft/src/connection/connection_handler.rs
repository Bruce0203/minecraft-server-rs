use std::io::{Error, ErrorKind, Result, Write};

use bytes::{Buf, BytesMut};
use common_server::{
    selector::{ConnectionHandler, Socket},
    var_int::VarIntRead,
};

use crate::{player::Player, protocol::v1_20_4::V1_20_4, server::Server};

use super::packet_read_handler::PacketReadHandler;

impl ConnectionHandler<Player> for Server {
    fn handle_connection_accept(&mut self) -> Player {
        Player::default()
    }

    fn handle_connection_read(&mut self, socket: &mut Socket<Player>, buf: &[u8]) -> Result<()> {
        let value = &mut BytesMut::from(buf);
        while !value.is_empty() {
            let packet_len = value.reader().read_var_i32()?;
            if value.len() < packet_len as usize {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "actual packet length is short than enough",
                ));
            }
            match socket.connection.session_relay.protocol_id {
                0 => {
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
        }
        Ok(())
    }

    fn handle_connection_closed(&mut self, _socket: &mut Socket<Player>) {}

    fn handle_update(&mut self) {
    }
}
