use crate::{
    io::prelude::{Encoder, U8Write},
    net::prelude::{PacketHandler, PacketId, Server, Socket},
    server::prelude::{GamePlayer, GameServer},
};
use derive_more::{Deref, From};

#[derive(Debug, Deref, From)]
pub struct C2SSetHeldItem(pub SetHeldItem);

#[derive(Debug, Deref, From)]
pub struct S2CSetHeldItem(pub SetHeldItem);

#[derive(Debug)]
pub struct SetHeldItem {
    pub slot: u8,
}

impl Encoder for SetHeldItem {
    fn encode_to_write<W: std::io::prelude::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_u8(self.slot)?;
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
