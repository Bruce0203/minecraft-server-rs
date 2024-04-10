use std::io::Result;

use crate::io::prelude::{Buffer, Decoder, Encoder};

pub struct EnterCombat;

impl Encoder for EnterCombat {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        Ok(())
    }
}

impl Decoder for EnterCombat {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(EnterCombat)
    }
}
