use std::io::{Error, Result, Write, Cursor};

use crate::io::encoding::Encoder;
use crate::io::primitives::I64Read;

use crate::{server::prelude::{Server, Player}, protocol::prelude::{PacketHandler, PacketWriter}};

#[derive(Debug)]
pub struct PingRequest {
    payload: i64,
}

impl TryFrom<&mut Cursor<Vec<u8>>> for PingRequest {
    type Error = Error;

    fn try_from(value: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(PingRequest {
            payload: value.read_i64()?,
        })
    }
}

impl PacketHandler for PingRequest {
    fn handle_packet(&self, socket: &mut Player) -> Result<()> {
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

impl PacketWriter for PingResponse {
    fn get_packet_id(&self, _socket: &mut Player) -> Result<i32> {
        Ok(0x01)
    }
}

impl Encoder for PingResponse {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&i64::to_be_bytes(self.payload))?;
        Ok(())
    }
}
