use std::io::{Error, Write};

use bytes::{Buf, BufMut, BytesMut};
use common_server::selector::Socket;
use common_server::{encoder::Encoder, packet::PacketHandler};

use crate::player::Player;
use crate::server::Server;

pub struct PingRequest {
    payload: i64,
}

impl TryFrom<BytesMut> for PingRequest {
    type Error = Error;

    fn try_from(mut value: BytesMut) -> Result<Self, Self::Error> {
        Ok(PingRequest {
            payload: value.get_i64(),
        })
    }
}

impl PacketHandler<Player, Server> for PingRequest {
    fn handle_packet(&self, server: &mut Server, player: &mut Socket<Player>) {
        player.stream.write_all(
            PingResponse {
                payload: self.payload,
            }
            .encode()
            .as_mut(),
        );
    }
}

pub struct PingResponse {
    payload: i64,
}

impl TryFrom<BytesMut> for PingResponse {
    type Error = Error;

    fn try_from(mut value: BytesMut) -> Result<Self, Self::Error> {
        Ok(PingResponse {
            payload: value.get_i64(),
        })
    }
}

impl Encoder for PingResponse {
    fn encode_to_bytes(&self, bytes: &mut BytesMut) {
        bytes.writer().write_all(&i64::to_be_bytes(self.payload));
    }
}

impl PacketHandler<Player, Server> for PingResponse {
    fn handle_packet(&self, server: &mut Server, player: &mut Socket<Player>) {}
}
