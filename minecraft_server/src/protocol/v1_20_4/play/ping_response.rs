use std::io::Result;

use crate::io::prelude::{Buffer, Decoder, Encoder, I16Read, I16Write, I64Read, I64Write};

#[derive(Debug)]
pub struct PingResponse {
    payload: i64,
}

impl Encoder for PingResponse {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_i64(self.payload)?;
        Ok(())
    }
}

impl Decoder for PingResponse {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(PingResponse {
            payload: reader.read_i64()?,
        })
    }
}
