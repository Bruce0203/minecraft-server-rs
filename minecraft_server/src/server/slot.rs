use serde::{Deserialize, Serialize};

use crate::io::prelude::{Encoder, NbtNetworkWrite, U8Write, VarIntWrite};

#[derive(Debug, Serialize, Deserialize)]
pub struct Slot {
    item_id: i32,
    item_count: u8,
    nbt: SlotData,
}

impl Encoder for Slot {
    fn encode_to_write<W: std::io::prelude::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_var_i32(self.item_id)?;
        writer.write_u8(self.item_count)?;
        writer.write_network_nbt(&self.nbt)?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlotData {}
