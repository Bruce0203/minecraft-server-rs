use std::io::{Cursor, Error, ErrorKind};

use crate::io::{primitives::U16Read, var_int::VarIntRead, var_string::VarStringRead};

use crate::{protocol::prelude::{ConnectionState, PacketHandler}, server::prelude::{Server, Player}};

#[derive(Debug)]
pub struct HandShake {
    protocol_version: i32,
    server_address: String,
    server_port: u16,
    next_state: NextState,
}

impl TryFrom<&mut Cursor<Vec<u8>>> for HandShake {
    type Error = Error;

    fn try_from(mut value: &mut Cursor<Vec<u8>>) -> std::result::Result<Self, Self::Error> {
        Ok(HandShake {
            protocol_version: value.read_var_i32()?,
            server_address: value.read_var_string::<255>()?,
            server_port: value.read_u16()?,
            next_state: NextState::try_from(value)?,
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

impl TryFrom<&mut Cursor<Vec<u8>>> for NextState {
    type Error = Error;

    fn try_from(value: &mut Cursor<Vec<u8>>) -> std::result::Result<Self, Self::Error> {
        Ok(match value.read_var_i32()? {
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

impl<'server> PacketHandler for HandShake {
    fn handle_packet(&self, value: &mut Player) -> std::io::Result<()> {
        let session_relay = &mut value.session_relay;
        session_relay.connection_state = Into::into(&self.next_state);
        session_relay.protocol_id = self.protocol_version;
        println!("HandShake!");
        Ok(())
    }
}
