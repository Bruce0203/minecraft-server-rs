use std::io::{Cursor, Error, Result, Write};

use mc_io::encoding::Encoder;
use mc_io::var_string::VarStringWrite;

use crate::connection::packet_handler::PacketHandler;
use crate::connection::packet_writer::PacketWriter;
use crate::connection::player::Player;
use crate::server::server_status::ServerStatus;
use crate::server::Server;

#[derive(Debug)]
pub struct StatusRequest {}

impl TryFrom<&mut Cursor<Vec<u8>>> for StatusRequest {
    type Error = Error;

    fn try_from(_value: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(StatusRequest {})
    }
}

impl StatusRequest {
    pub fn new() -> StatusRequest {
        StatusRequest {}
    }
}

pub struct StatusResponse<'a> {
    server_status: &'a ServerStatus,
}

impl<'a> PacketWriter for StatusResponse<'a> {
    fn get_packet_id(&self, socket: &mut Player) -> Result<i32> {
        Ok(0x00)
    }
}

impl<'a> Encoder for StatusResponse<'a> {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let server_status_data = serde_json::to_string(&self.server_status)?;
        writer.write_var_string(server_status_data.as_str())?;
        Ok(())
    }
}

impl PacketHandler<Server, Player> for StatusRequest {
    fn handle_packet(&self, server: &mut Server, socket: &mut Player) -> Result<()> {
        let status_response = StatusResponse {
            server_status: &server.server_status,
        };
        status_response.send_packet(socket)?;
        Ok(())
    }
}
