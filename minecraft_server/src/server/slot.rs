use std::io::{prelude::Write, Result};

use serde::{Deserialize, Serialize};

use crate::io::prelude::{
    Buffer, Decoder, DecoderDeref, Encoder, EncoderDeref, NbtNetworkWrite, OptionRead, OptionWrite,
    U8Read, U8Write, VarIntRead, VarIntWrite,
};

#[derive(Debug, Clone)]
pub struct Slot(pub Option<SlotData>);

impl Slot {
    pub const None: Slot = Slot(None);
}

impl Encoder for Slot {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        self.0.encode_to_buffer(buf)?;
        Ok(())
    }
}

impl Decoder for Slot {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(Slot(reader.read_option()?))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlotData {
    item_id: i32,
    item_count: u8,
    nbt: SlotNbt,
}

impl !EncoderDeref for SlotData {}
impl !DecoderDeref for SlotData {}

impl Encoder for SlotData {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(self.item_id)?;
        buf.write_u8(self.item_count)?;
        buf.write_network_nbt(&self.nbt)?;
        Ok(())
    }
}

impl Decoder for SlotData {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(SlotData {
            item_id: reader.read_var_i32()?,
            item_count: reader.read_u8()?,
            nbt: SlotNbt {},
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlotNbt {}
