use std::io::prelude::Write;
use std::io::{Cursor, Error, ErrorKind, Read, Result};

use crate::io::prelude::{Decoder, Encoder, VarIntWrite, VarStringWrite};
use crate::io::primitives::U16Write;
use crate::io::{primitives::U16Read, var_int::VarIntRead, var_string::VarStringRead};

use crate::net::prelude::{PacketHandler, Player};
use crate::server::prelude::Server;
use crate::{
    protocol::prelude::ConnectionState,
};

#[derive(Debug)]
pub struct HandShake {
    protocol_version: i32,
    server_address: String,
    server_port: u16,
    next_state: NextState,
}

impl Decoder for HandShake {
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(HandShake {
            protocol_version: reader.read_var_i32()?,
            server_address: reader.read_var_string::<255>()?,
            server_port: reader.read_u16()?,
            next_state: NextState::decode_from_read(reader)?,
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

impl Encoder for NextState {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_var_i32(match self {
            NextState::Status => 1,
            NextState::Login => 2,
        })?;
        Ok(())
    }
}

impl Decoder for NextState {
    fn decode_from_read<R: std::io::prelude::Read>(reader: &mut R) -> Result<Self> {
        Ok(match reader.read_var_i32()? {
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
    fn handle_packet(&self, server: &mut Server, value: &mut Player) -> Result<()> {
        let session_relay = &mut value.session_relay;
        session_relay.connection_state = Into::into(&self.next_state);
        session_relay.protocol_id = self.protocol_version;
        println!("handhsake");
        Ok(())
    }
}

impl Encoder for HandShake {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_var_i32(self.protocol_version)?;
        writer.write_var_string(&self.server_address)?;
        writer.write_u16(self.server_port)?;
        self.next_state.encode_to_write(writer)?;
        Ok(())
    }
}
