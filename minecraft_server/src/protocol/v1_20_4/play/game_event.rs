use std::io::Result;

use crate::io::prelude::{Buffer, Decoder, Encoder, F32Read, F32Write, U8Read, U8Write};

#[derive(Debug)]
pub struct GameEvent {
    event: u8,
    value: f32,
}

impl Encoder for GameEvent {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_u8(self.event)?;
        buf.write_f32(self.value)?;
        Ok(())
    }
}

impl Decoder for GameEvent {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(GameEvent {
            event: reader.read_u8()?,
            value: reader.read_f32()?,
        })
    }
}
