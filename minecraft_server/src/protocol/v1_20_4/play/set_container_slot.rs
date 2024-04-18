use std::io::{prelude::Write, Result};

use crate::{
    io::prelude::{
        Buffer, Decoder, Encoder, I16Read, I16Write, NbtNetworkWrite, OptionWrite, U8Read, U8Write,
        VarIntRead, VarIntWrite,
    },
    net::prelude::PacketWriter,
    server::slot::Slot,
};

#[derive(Debug)]
pub struct SetContainerSlot {
    pub window_id: u8,
    pub state_id: i32,
    pub slot: i16,
    pub slot_data: Slot,
}

impl Encoder for SetContainerSlot {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_u8(self.window_id)?;
        buf.write_var_i32(self.state_id)?;
        buf.write_i16(self.slot)?;
        self.slot_data.encode_to_buffer(buf)?;
        Ok(())
    }
}

impl Decoder for SetContainerSlot {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(SetContainerSlot {
            window_id: reader.read_u8()?,
            state_id: reader.read_var_i32()?,
            slot: reader.read_i16()?,
            slot_data: Slot::decode_from_read(reader)?,
        })
    }
}
