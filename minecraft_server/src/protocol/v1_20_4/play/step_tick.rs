use std::io::Result;

use crate::io::prelude::{Buffer, Decoder, Encoder, VarIntRead, VarIntWrite};

#[derive(Debug)]
pub struct StepTick {
    tick_steps: i32,
}

impl Encoder for StepTick {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(self.tick_steps)?;
        Ok(())
    }
}

impl Decoder for StepTick {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(StepTick {
            tick_steps: reader.read_var_i32()?,
        })
    }
}
