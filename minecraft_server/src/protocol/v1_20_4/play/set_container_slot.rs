use std::io::{prelude::Write, Result};

use crate::{io::prelude::{Encoder, U8Write, VarIntWrite, I16Write, NbtNetworkWrite}, server::slot::Slot, net::prelude::PacketWriter};

pub struct SetContainerSlot {
    window_id: u8,
    state_id: i32,
    slot: i16,
    slot_data: Slot,
}

impl Encoder for SetContainerSlot {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.window_id)?;
        writer.write_var_i32(self.state_id)?;
        writer.write_i16(self.slot)?;
        self.slot_data.encode_to_write(writer)?;
        Ok(())
    }
}

