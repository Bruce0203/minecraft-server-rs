use crate::{io::prelude::{Encoder, U8Write}, net::prelude::{PacketIdentifier, Player}};


pub struct SetHeldItem {
    pub slot: u8,
}

impl Encoder for SetHeldItem {
    fn encode_to_write<W: std::io::prelude::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_u8(self.slot)?;
        Ok(())
    }
}

impl PacketIdentifier for SetHeldItem {
    fn get_packet_id(&self, player: &mut Player) -> std::io::Result<i32> {
        Ok(0x51)
    }
}
