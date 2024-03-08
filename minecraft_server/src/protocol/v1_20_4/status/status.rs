use std::io::{Cursor, Error, Result, Write};

use crate::io::prelude::{Cache, Decoder, Encoder};

use crate::net::prelude::{PacketHandler, PacketIdentifier, PacketWriter, Socket};
use crate::server::prelude::{Server, GamePlayer};
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

impl<'a> PacketIdentifier<GamePlayer> for StatusResponse<'a> {
    fn get_packet_id(&self, _socket: &mut Socket<GamePlayer>) -> Result<i32> {
        Ok(0x00)
    }
}

impl<'a> Encoder for StatusResponse<'a> {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.server_status.encode_to_write(writer)?;
        Ok(())
    }
}

impl PacketHandler<GamePlayer> for StatusRequest {
    fn handle_packet(&self, server: &mut Server, player: &mut Socket<GamePlayer>) -> Result<()> {
        StatusResponse {
            server_status: &mut server.server_status,
        }
        .send_packet(player)?;
        Ok(())
    }
}
