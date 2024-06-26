use std::io::{Cursor, Error, Read, Result, Write};

use crate::io::prelude::{Buffer, Cache, Decoder, Encoder, I64Read};

use crate::net::prelude::{PacketHandler, PacketId, PacketWriter, Socket};
use crate::server::prelude::{GamePlayer, GameServer};
use crate::server::server_status::ServerStatus;

#[derive(Debug)]
pub struct StatusRequest {}

impl Decoder for StatusRequest {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(StatusRequest {})
    }
}

impl StatusRequest {
    pub fn new() -> StatusRequest {
        StatusRequest {}
    }
}

#[derive(Debug)]
pub struct StatusResponse<'a> {
    server_status: &'a Cache<ServerStatus>,
}

impl<'a> Encoder for StatusResponse<'a> {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        self.server_status.encode_to_buffer(buf)?;
        Ok(())
    }
}

impl PacketHandler<GameServer> for StatusRequest {
    fn handle_packet(
        &self,
        server: &mut GameServer,
        player: &mut Socket<GamePlayer>,
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
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(PingRequest {
            payload: reader.read_i64()?,
        })
    }
}

impl PacketHandler<GameServer> for PingRequest {
    fn handle_packet(
        &self,
        server: &mut GameServer,
        socket: &mut Socket<GamePlayer>,
    ) -> Result<()> {
        let ping_response = PingResponseStatus {
            payload: self.payload,
        };
        ping_response.send_packet(socket)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct PingResponseStatus {
    payload: i64,
}

impl Encoder for PingResponseStatus {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        buf.write_all(&i64::to_be_bytes(self.payload))?;
        Ok(())
    }
}

impl Decoder for PingResponseStatus {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(PingResponseStatus {
            payload: reader.read_i64()?,
        })
    }
}
