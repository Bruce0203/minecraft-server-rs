use std::io::Result;

use crate::io::prelude::{Buffer, Decoder, Encoder, I32Read, I32Write, VarIntRead};

#[derive(Debug)]
pub struct PongC2s {
    pub id: i32,
}

impl Encoder for PongC2s {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_i32(self.id)?;
        Ok(())
    }
}

impl Decoder for PongC2s {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(PongC2s {
            id: reader.read_i32()?,
        })
    }
}

