use std::io::Result;

use crate::io::prelude::{Buffer, Decoder, Encoder, VarIntRead, VarIntWrite};

#[derive(Debug)]
pub struct Ping {
    id: i32,
}

impl Encoder for Ping {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(self.id)?;
        Ok(())
    }
}

impl Decoder for Ping {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(Ping {
            id: reader.read_var_i32()?,
        })
    }
}
