use std::io::{prelude::Write, Result};

use crate::{
    io::prelude::{OptionWrite, Encoder, I16Write, NbtNetworkWrite, U8Write, VarIntWrite},
    net::prelude::PacketWriter,
    server::slot::Slot,
};

pub struct SetContainerSlot {
    pub window_id: u8,
    pub state_id: i32,
    pub slot: i16,
    pub slot_data: Slot,
}

impl Encoder for SetContainerSlot {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.window_id)?;
        writer.write_var_i32(self.state_id)?;
        writer.write_i16(self.slot)?;
        writer.write_option(&self.slot_data)?;
        Ok(())
    }
}
