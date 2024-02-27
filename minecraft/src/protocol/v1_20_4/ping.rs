use std::io::{Error, Write, Result};
use bytes::{Buf, BufMut, BytesMut};
use common_server::encoder::PacketWriter;
use common_server::selector::Socket;
use common_server::{encoder::Encoder, packet::PacketHandler};

use crate::player::Player;
use crate::server::Server;

pub struct PingRequest {
    payload: i64,
}

impl TryFrom<BytesMut> for PingRequest {
    type Error = Error;

    fn try_from(mut value: BytesMut) -> Result<Self> {
        Ok(PingRequest {
            payload: value.get_i64(),
        })
    }
}

impl PacketHandler<Server, Player> for PingRequest {
    fn handle_packet(&self, _server: &mut Server, player: &mut Socket<Player>) -> Result<()> {
        player.stream.write_all(
            PingResponse {
                payload: self.payload,
            }
            .encode()
            .as_mut(),
        ).unwrap();
        Ok(())
    }
}

pub struct PingResponse {
    payload: i64,
}

impl PacketWriter<Player> for PingResponse {
    fn send_packet(&self, socket: &mut Socket<Player>) -> Result<()> {
        todo!()
    }
}

impl Encoder for PingResponse {
    fn encode_to_write<W: Write>(&self, write: &mut W) {
        write.write_all(&i64::to_be_bytes(self.payload));
    }
}

impl PacketHandler<Server, Player> for PingResponse {
    fn handle_packet(&self, server: &mut Server, player: &mut Socket<Player>) -> Result<()> {
        todo!()
    }
}
