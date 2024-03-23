use std::io::prelude::Write;
use std::io::{Cursor, Error, ErrorKind, Read, Result};

use crate::io::prelude::{
    Buffer, Decoder, Encoder, U16Read, U16Write, VarIntRead, VarIntWrite, VarStringRead,
    VarStringWrite,
};

use crate::net::prelude::{PacketHandler, PacketId, Socket};
use crate::server::prelude::{ConnectionState, GamePlayer, GameServer};

#[derive(Debug)]
pub struct HandShake {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: NextState,
}

impl Decoder for HandShake {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
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
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        buf.write_var_i32(match self {
            NextState::Status => 1,
            NextState::Login => 2,
        })?;
        Ok(())
    }
}

impl Decoder for NextState {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
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

impl PacketHandler<GameServer> for HandShake {
    fn handle_packet(&self, server: &mut GameServer, value: &mut Socket<GamePlayer>) -> Result<()> {
        let session_relay = &mut value.session_relay;
        session_relay.connection_state = Into::into(&self.next_state);
        session_relay.protocol_id = self.protocol_version;
        Ok(())
    }
}

impl Encoder for HandShake {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        buf.write_var_i32(self.protocol_version)?;
        buf.write_var_string(&self.server_address)?;
        buf.write_u16(self.server_port)?;
        self.next_state.encode_to_buffer(buf)?;
        Ok(())
    }
}
