use bytes::{Buf, BytesMut};
use std::io::{Error, ErrorKind, Result};

use crate::{
    minecraft::{packet_handler::PacketHandler, player::Player, session_relay::ConnectionState},
    selector::Socket,
    var_int::VarIntRead,
};

use super::{handshake::HandShake, login_start::LoginStart};

impl Socket<Player> {
    pub fn handle_packet(&mut self, value: BytesMut) -> Result<()> {
        let mut reader = value.reader();
        let packet_id = reader.read_var_i32()?;
        let mut player = &mut *self.connection;
        let connection_state = &player.session_relay.connection_state;
        let mut bytes = reader.into_inner();
        match (connection_state, packet_id) {
            (ConnectionState::HandShake, 0) => HandShake::try_from(bytes)?.handle_packet(player),
            (ConnectionState::Login, 0) => LoginStart::try_from(bytes)?.handle_packet(player),
            (_) => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("{:?}[{:?}] not exists", connection_state, packet_id),
                ))
            }
        };
        Ok(())
    }
}
