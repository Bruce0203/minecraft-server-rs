use std::io::Result;

use crate::io::prelude::{Buffer, Decoder, Encoder};

#[derive(Debug)]
pub struct UpdateReceipeBook {
    //TODO wip
}

impl Encoder for UpdateReceipeBook {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        println!("hi");
        Ok(())
    }
}

impl Decoder for UpdateReceipeBook {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(UpdateReceipeBook {})
    }
}
