use std::io::{prelude::Write, Result};

use serde::{Deserialize, Serialize};

use crate::io::prelude::{
    DecoderDeref, Encoder, EncoderDeref, NbtNetworkWrite, OptionWrite, U8Write, VarIntWrite,
};

pub type Slot = Option<SlotData>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlotData {
    item_id: i32,
    item_count: u8,
    nbt: SlotNbt,
}

impl !EncoderDeref for SlotData {}
impl !DecoderDeref for SlotData {}

impl Encoder for SlotData {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        buf.write_var_i32(self.item_id)?;
        buf.write_u8(self.item_count)?;
        buf.write_network_nbt(&self.nbt)?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlotNbt {}
