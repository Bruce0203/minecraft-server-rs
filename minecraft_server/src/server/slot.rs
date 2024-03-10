use std::io::{prelude::Write, Result};

use serde::{Deserialize, Serialize};

use crate::io::prelude::{Encoder, NbtNetworkWrite, U8Write, VarIntWrite, OptionWrite};

pub type Slot = Option<SlotData>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlotData {
    item_id: i32,
    item_count: u8,
    nbt: SlotNbt,
}

impl Encoder for Slot {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_option(self)?;
        Ok(())
    }
}

impl Encoder for SlotData {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_var_i32(self.item_id)?;
        writer.write_u8(self.item_count)?;
        writer.write_network_nbt(&self.nbt)?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlotNbt {}
