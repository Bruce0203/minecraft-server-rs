use std::io::{Cursor, Error, Read, Result, Write};

use crate::io::prelude::{Cache, Decoder, Encoder, I64Read};

use crate::net::prelude::{PacketHandler, PacketId, PacketWriter, Socket};
use crate::server::prelude::{LoginPlayer, LoginServer};
use crate::server::server_status::ServerStatus;

#[derive(Debug)]
pub struct StatusRequest {}

impl Decoder for StatusRequest {
    fn decode_from_read<R: std::io::prelude::Read>(reader: &mut R) -> Result<Self> {
        Ok(StatusRequest {})
    }
}

impl StatusRequest {
    pub fn new() -> StatusRequest {
        StatusRequest {}
    }
}

pub struct StatusResponse<'a> {
    server_status: &'a Cache<ServerStatus>,
}

impl<'a> Encoder for StatusResponse<'a> {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.server_status.encode_to_write(writer)?;
        Ok(())
    }
}

impl PacketHandler<LoginServer> for StatusRequest {
    fn handle_packet(
        &self,
        server: &mut LoginServer,
        player: &mut Socket<LoginPlayer>,
    ) -> Result<()> {
        StatusResponse {
            server_status: &mut server.server_status,
        }
        .send_packet(player)?;
        Ok(())
    }
}

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

impl PacketHandler<LoginServer> for PingRequest {
    fn handle_packet(
        &self,
        server: &mut LoginServer,
        socket: &mut Socket<LoginPlayer>,
    ) -> Result<()> {
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

impl Encoder for PingResponse {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&i64::to_be_bytes(self.payload))?;
        Ok(())
    }
}
