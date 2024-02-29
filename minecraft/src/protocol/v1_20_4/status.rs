use std::io::{Error, Result, Write};

use bytes::BytesMut;
use common_server::encoding::Encoder;
use common_server::packet::PacketHandler;
use common_server::selector::Socket;
use common_server::var_string::VarStringWrite;
use json::JsonValue;

use crate::connection::packet_writer::PacketWriter;
use crate::player::Player;
use crate::server::server_status::ServerStatus;
use crate::server::Server;

#[derive(Debug)]
pub struct StatusRequest {}

impl TryFrom<&mut BytesMut> for StatusRequest {
    type Error = Error;

    fn try_from(_value: &mut BytesMut) -> Result<Self> {
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
        let server_status_data: &JsonValue = &self.server_status.into();
        writer.write_var_string(server_status_data.dump().as_str())?;
        Ok(())
    }
}

impl PacketHandler<Server, Player> for StatusRequest {
    fn handle_packet(&self, server: &mut Server, socket: &mut Player) -> Result<()> {
        println!("{:#?}", self);
        let status_response = StatusResponse {
            server_status: &server.server_status,
        };
        status_response.send_packet(socket)?;
        Ok(())
    }
}
