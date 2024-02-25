use std::{io::Error, ops::Deref};

use bytes::{Buf, BytesMut};
use common_server::{selector::Socket, var_int::VarIntRead};

use crate::{connection_state::ConnectionState, packet_handler::PacketHandler, player::Player};

pub struct HandShake {
    protocol_version: i32,
    next_state: NextState,
}

impl TryFrom<BytesMut> for HandShake {
    type Error = Error;

    fn try_from(mut value: BytesMut) -> Result<Self, Self::Error> {
        Ok(HandShake {
            protocol_version: value.get_i32(),
            next_state: NextState::try_from(value)?,
        })
    }
}

pub enum NextState {
    Status,
    Login,
}

impl From<&NextState> for ConnectionState {
    fn from(value: &NextState) -> Self {
        match value {
            NextState::Status => ConnectionState::Status,
            NextState::Login => ConnectionState::Login,
        }
    }
}
impl TryFrom<BytesMut> for NextState {
    type Error = Error;

    fn try_from(value: BytesMut) -> Result<Self, Self::Error> {
        match value.reader().read_var_i32()? {
            1 => Ok(NextState::Status),
            2 => Ok(NextState::Login),
            n => {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("NextState is {:?}", n),
                ))
            }
        }
    }
}

impl PacketHandler for HandShake {
    fn handle_packet(&self, value: &mut Socket<Player>) {
        value.connection.connection_state = (&self.next_state).into();
    }
}
