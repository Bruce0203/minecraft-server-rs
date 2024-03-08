use std::io::{Cursor, Error, Result, Write};

use crate::io::prelude::{Cache, Decoder, Encoder};

use crate::net::prelude::{PacketHandler, PacketIdentnifier, Player};
use crate::server::prelude::Server;
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

impl<'a> PacketIdentnifier for StatusResponse<'a> {
    fn get_packet_id(&self, _socket: &mut Player) -> Result<i32> {
        Ok(0x00)
    }
}

impl<'a> Encoder for StatusResponse<'a> {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.server_status.encode_to_write(writer)?;
        Ok(())
    }
}

impl PacketHandler for StatusRequest {
    fn handle_packet(&self, server: &mut Server, player: &mut Player) -> Result<()> {
        StatusResponse {
            server_status: &mut server.server_status,
        }
        .send_packet(player)?;
        println!("status");
        Ok(())
    }
}
