use std::io::{Error, ErrorKind, Result};
use bytes::{Buf, BytesMut};
use common_server::{selector::Socket, var_int::VarIntRead, var_string::VarStringRead};

use crate::{
    packet_handler::PacketHandler, player::Player, session_relay::ConnectionState,
};

#[derive(Debug)]
pub struct HandShake {
    protocol_version: i32,
    server_address: String,
    server_port: u16,
    next_state: NextState,
}

impl TryFrom<BytesMut> for HandShake {
    type Error = Error;

    fn try_from(mut value: BytesMut) -> Result<Self> {
        let mut reader = value.reader();
        Ok(HandShake {
            protocol_version: reader.read_var_i32()?,
            server_address: reader.read_var_string::<255>()?,
            server_port: reader.get_mut().get_u16(),
            next_state: NextState::try_from(reader.into_inner())?,
        })
    }
}

#[derive(Debug)]
pub enum NextState {
    Status,
    Login,
}

impl From<&NextState> for ConnectionState {
    fn from(value: &NextState) -> Self {
        match value {
            NextState::Login => ConnectionState::Login,
            NextState::Status => ConnectionState::Status,
        }
    }
}

impl TryFrom<BytesMut> for NextState {
    type Error = Error;

    fn try_from(mut value: BytesMut) -> std::result::Result<Self, Self::Error> {
        Ok(match value.reader().read_var_i32()? {
            1 => NextState::Status,
            2 => NextState::Login,
            n => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("NextState is {}", n),
                ))
            }
        })
    }
}

impl PacketHandler for HandShake {
    fn handle_packet(&self, value: &mut Socket<Player>) {
        value.connection.session_relay.connection_state = (&self.next_state).into();
    }
}
