use std::io::prelude::Read;
use std::io::{Cursor, Error, Result, Write};

use crate::io::prelude::{Decoder, Encoder, I64Read};
use crate::net::prelude::{PacketHandler, PacketIdentifier, PacketWriter, Socket};
use crate::server::prelude::{Server, GamePlayer};

#[derive(Debug)]
pub struct PingRequest {
    payload: i64,
}

impl Decoder for PingRequest {
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(PingRequest {
            payload: reader.read_i64()?,
        })
    }
}

impl PacketHandler<GamePlayer> for PingRequest {
    fn handle_packet(&self, server: &mut Server, socket: &mut Socket<GamePlayer>) -> Result<()> {
        let ping_response = PingResponse {
            payload: self.payload,
        };
        ping_response.send_packet(socket)?;
        Ok(())
    }
}

pub struct PingResponse {
    payload: i64,
}

impl PacketIdentifier<GamePlayer> for PingResponse {
    fn get_packet_id(&self, _socket: &mut GamePlayer) -> Result<i32> {
        Ok(0x01)
    }
}

impl Encoder for PingResponse {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&i64::to_be_bytes(self.payload))?;
        Ok(())
    }
}
