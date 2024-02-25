use std::io::{Error, ErrorKind, Result};

use bytes::{Buf, BytesMut};
use common_server::{selector::Socket, var_int::VarIntRead};

use crate::{packet_read_handler::PacketReadHandler, session_relay::ConnectionState, player::Player, packet_handler::PacketHandler};

use self::{handshake::HandShake, login_start::LoginStart};

pub mod handshake;
pub mod login_start;
pub mod ping;
pub mod status;

pub struct V1_20_1;

impl PacketReadHandler for V1_20_1 {
    fn handle_packet_read(socket: &mut Socket<Player>, value: BytesMut) -> Result<()> {
        let mut reader = value.reader();
        let packet_id = reader.read_var_i32()?;
        let connection_state = &socket.connection.session_relay.connection_state;
        let mut bytes = reader.into_inner();
        match (connection_state, packet_id) {
            (ConnectionState::HandShake, 0) => HandShake::try_from(bytes)?.handle_packet(socket),
            (ConnectionState::Login, 0) => LoginStart::try_from(bytes)?.handle_packet(socket),
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
