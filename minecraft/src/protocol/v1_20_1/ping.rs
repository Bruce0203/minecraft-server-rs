use std::io::{Error, Write};

use bytes::{Buf, BufMut, BytesMut};
use common_server::selector::Socket;

use crate::{packet_encoder::PacketEncoder, packet_handler::PacketHandler, player::Player};

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

impl PacketHandler for PingRequest {
    fn handle_packet(&self, system: &mut Socket<Player>) {
        system.stream.write_all(
            PingResponse {
                payload: self.payload,
            }
            .encode_packet()
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

impl PacketEncoder for PingResponse {
    fn encode_packet_to_bytes(&self, bytes: &mut BytesMut) {
        bytes.writer().write_all(&i64::to_be_bytes(self.payload));
    }
}

impl PacketHandler for PingResponse {
    fn handle_packet(&self, value: &mut Socket<Player>) {}
}
