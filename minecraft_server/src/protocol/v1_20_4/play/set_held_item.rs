use std::io::Result;

use crate::{
    io::prelude::{Buffer, Decoder, DecoderDeref, Encoder, EncoderDeref, U8Read, U8Write},
    net::prelude::{PacketHandler, PacketId, Server, Socket},
    server::prelude::{GamePlayer, GameServer},
};
use derive_more::{Deref, From, Into};

#[derive(Debug, Deref, From, Into)]
pub struct SetHeldItemC2s(pub SetHeldItem);

impl DecoderDeref for SetHeldItemC2s {}
impl EncoderDeref for SetHeldItemC2s {}

#[derive(Debug, Deref, From, Into)]
pub struct SetHeldItemS2c(pub SetHeldItem);

impl DecoderDeref for SetHeldItemS2c {}
impl EncoderDeref for SetHeldItemS2c {}

#[derive(Debug)]
pub struct SetHeldItem {
    pub slot: u8,
}

impl Encoder for SetHeldItem {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_u8(self.slot)?;
        Ok(())
    }
}

impl Decoder for SetHeldItem {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(SetHeldItem {
            slot: reader.read_u8()?,
        })
    }
}

impl PacketHandler<GameServer> for SetHeldItem {
    fn handle_packet(
        &self,
        server: &mut GameServer,
        player: &mut Socket<<GameServer as Server>::Player>,
    ) -> Result<()> {
        todo!()
    }
}
