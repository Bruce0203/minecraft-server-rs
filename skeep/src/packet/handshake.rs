use std::{
    io::Error,
    io::{Result, Write},
    mem::size_of,
};

use bytes::{Buf, BufMut, BytesMut};
use common_server::{
    encoder::Encoder, packet::PacketHandler, selector::Socket,
    var_int::VarIntRead,
};

use crate::{connection_state::ConnectionState, player::Player};

pub struct HandShake {
    protocol_version: i32,
    next_state: NextState,
}

impl HandShake {
    pub fn new(protocol_version: i32, next_state: NextState) -> HandShake {
        HandShake {
            protocol_version,
            next_state,
        }
    }
}

impl TryFrom<BytesMut> for HandShake {
    type Error = Error;

    fn try_from(mut value: BytesMut) -> Result<Self> {
        Ok(HandShake {
            protocol_version: value.get_i32(),
            next_state: NextState::try_from(value)?,
        })
    }
}

impl Encoder for HandShake {
    fn encode_to_bytes(&self, bytes: &mut BytesMut) {
        let mut writer = bytes.writer();
        writer.write_all(&i32::to_be_bytes(self.protocol_version)).unwrap();
        writer.write_all(&[size_of::<NextState>() as u8]).unwrap();
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

impl Encoder for NextState {
    fn encode_to_bytes(&self, bytes: &mut BytesMut) {
        let mut writer = bytes.writer();
        writer.write_all(&[size_of::<NextState>() as u8]).unwrap();
    }
}

impl TryFrom<BytesMut> for NextState {
    type Error = Error;

    fn try_from(value: BytesMut) -> Result<Self> {
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

impl PacketHandler<Player> for HandShake {
    fn handle_packet(&self, value: &mut Socket<Player>) {
        value.connection.connection_state = (&self.next_state).into();
    }
}
