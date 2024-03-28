use std::io::Result;

use crate::io::prelude::{BoolRead, Buffer, Decoder, Encoder, F32Read, F32Write, WriteBool};

#[derive(Debug)]
pub struct SetTickingState {
    tick_rate: f32,
    is_frozen: bool,
}

impl Encoder for SetTickingState {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_f32(self.tick_rate)?;
        buf.write_bool(self.is_frozen)?;
        Ok(())
    }
}

impl Decoder for SetTickingState {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(SetTickingState {
            tick_rate: reader.read_f32()?,
            is_frozen: reader.read_bool()?,
        })
    }
}
