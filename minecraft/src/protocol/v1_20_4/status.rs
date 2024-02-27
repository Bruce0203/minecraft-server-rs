use std::io::{Result, Write};
use std::time::Duration;

use backtrace::Backtrace;
use bytes::{BufMut, BytesMut};
use common_server::encoder::{Encoder, PacketWriter};
use common_server::packet::PacketHandler;
use common_server::selector::Socket;
use common_server::var_int::VarIntWrite;
use common_server::var_string::VarStringWrite;
use json::{object, JsonValue};

use crate::chat::Chat;
use crate::player::Player;
use crate::server::server_status::ServerStatus;
use crate::server::Server;

pub struct StatusRequest {}

impl StatusRequest {
    pub fn new() -> StatusRequest {
        StatusRequest {}
    }
}

pub struct StatusResponse<'a> {
    server_status: &'a ServerStatus,
}

impl<'a> Encoder for StatusResponse<'a> {
    fn encode_to_write<W: Write>(&self, writer: &mut W) {
    }
}

impl<'a> PacketWriter<Player> for StatusResponse<'a> {
    fn send_packet(&self, socket: &mut Socket<Player>) -> Result<()> {
        let payload = self.encode();
        socket.stream.write_all(&[0x00])?;
        socket.stream.write_var_i32(payload.len() as i32)?;
        socket.stream.write_all(&payload)?;
        Ok(())
    }
}

impl PacketHandler<Server, Player> for StatusRequest {
    fn handle_packet(&self, server: &mut Server, player: &mut Socket<Player>) -> Result<()> {
        let status_response = StatusResponse {
            server_status: &server.server_status,
        };
        status_response.send_packet(player)?;
        Ok(())
    }
}
