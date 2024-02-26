use bytes::{Buf, BytesMut};
use common_server::{
    packet::PacketHandler, selector::Socket, var_int::VarIntRead, var_string::VarStringRead,
};
use std::io::{Error, ErrorKind, Result};

use crate::{
    player::Player,
    session_relay::{self, ConnectionState}, server::Server,
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

    fn try_from(value: BytesMut) -> std::result::Result<Self, Self::Error> {
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

impl PacketHandler<Player, Server> for HandShake {
    fn handle_packet(&self, server: &mut Server, value: &mut Socket<Player>) {
        println!("handshake!");
        let session_relay = &mut value.connection.session_relay;
        session_relay.connection_state = Into::into(&self.next_state);
        session_relay.protocol_id = self.protocol_version;
    }
}
