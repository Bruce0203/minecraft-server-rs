use std::io::Error;

use bytes::BytesMut;
use common_server::encoder::Encoder;
use common_server::packet::PacketHandler;
use common_server::selector::Socket;

use crate::chat::Chat;
use crate::player::Player;
use crate::server::{Players, Server, ServerStatus};

pub struct StatusRequest {}

impl StatusRequest {
    pub fn new() -> StatusRequest {
        StatusRequest {}
    }
}

pub struct StatusResponse {
    server_status: ServerStatus,
}

impl TryFrom<BytesMut> for StatusResponse {
    type Error = Error;

    fn try_from(value: BytesMut) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl Encoder for StatusResponse {
    fn encode_to_bytes(&self, bytes: &mut BytesMut) {}
}

impl PacketHandler<Player, Server> for StatusRequest {
    fn handle_packet(&self, server: &mut Server, player: &mut Socket<Player>) {}
}
