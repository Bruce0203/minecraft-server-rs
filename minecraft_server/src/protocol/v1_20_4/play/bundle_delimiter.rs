use std::io::Result;

use crate::io::prelude::{Buffer, Decoder, Encoder};

#[derive(Debug)]
pub struct BundleDelimiter;

impl Decoder for BundleDelimiter {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(BundleDelimiter)
    }
}

impl Encoder for BundleDelimiter {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        Ok(())
    }
}
