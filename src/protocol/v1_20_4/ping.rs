use std::io::{Cursor, Error, Result, Write};

use crate::io::prelude::{Encoder, I64Read};
use crate::net::prelude::{PacketHandler, PacketIdentnifier, Player};
use crate::server::prelude::Server;

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
    fn handle_packet(&self, server: &mut Server, socket: &mut Player) -> Result<()> {
        let ping_response = PingResponse {
            payload: self.payload,
        };
        ping_response.send_packet(socket)?;
        println!("ping");
        Ok(())
    }
}

pub struct PingResponse {
    payload: i64,
}

impl PacketIdentnifier for PingResponse {
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
