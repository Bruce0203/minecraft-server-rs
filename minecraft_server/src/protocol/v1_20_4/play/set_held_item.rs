use crate::{
    io::prelude::{Encoder, EncoderDeref, U8Write},
    net::prelude::{PacketHandler, PacketId, Server, Socket},
    server::prelude::{GamePlayer, GameServer},
};
use derive_more::{Deref, From, Into};

#[derive(Debug, Deref, From, Into)]
pub struct C2SSetHeldItem(pub SetHeldItem);

#[derive(Debug, Deref, From, Into)]
pub struct S2CSetHeldItem(pub SetHeldItem);

impl EncoderDeref for S2CSetHeldItem {}
#[derive(Debug)]
pub struct SetHeldItem {
    pub slot: u8,
}

impl Encoder for SetHeldItem {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> std::io::Result<()> {
        buf.write_u8(self.slot)?;
        Ok(())
    }
}

impl PacketHandler<GameServer> for SetHeldItem {
    fn handle_packet(
        &self,
        server: &mut GameServer,
        player: &mut Socket<<GameServer as Server>::Player>,
    ) -> std::io::Result<()> {
        todo!()
    }
}
