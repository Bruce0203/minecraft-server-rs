use std::io::Result;

use crate::io::prelude::{Buffer, Decoder, Encoder, VarIntRead, VarIntWrite};

pub struct EndCombat {
    duration: i32,
}

impl Encoder for EndCombat {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(self.duration)?;
        Ok(())
    }
}

impl Decoder for EndCombat {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(EndCombat {
            duration: reader.read_var_i32()?,
        })
    }
}
