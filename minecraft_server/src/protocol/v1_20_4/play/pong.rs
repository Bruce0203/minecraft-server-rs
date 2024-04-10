use std::io::Result;

use crate::io::prelude::{Buffer, Decoder, Encoder, I32Read, I32Write, VarIntRead};

pub struct Pong {
    id: i32,
}

impl Encoder for Pong {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_i32(self.id)?;
        Ok(())
    }
}

impl Decoder for Pong {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(Pong {
            id: reader.read_i32()?,
        })
    }
}
