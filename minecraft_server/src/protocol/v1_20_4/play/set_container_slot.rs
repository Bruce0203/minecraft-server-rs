use std::io::{prelude::Write, Result};

use crate::{
    io::prelude::{Encoder, I16Write, NbtNetworkWrite, OptionWrite, U8Write, VarIntWrite},
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
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        buf.write_u8(self.window_id)?;
        buf.write_var_i32(self.state_id)?;
        buf.write_i16(self.slot)?;
        buf.write_option(&self.slot_data)?;
        Ok(())
    }
}
