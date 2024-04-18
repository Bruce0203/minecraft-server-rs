use std::io::Result;

use crate::{
    io::prelude::{Buffer, Decoder, Encoder},
    server::chat::Chat,
};

#[derive(Debug)]
pub struct Disconnect {
    reason: Chat,
}

impl Encoder for Disconnect {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        self.reason.encode_to_buffer(buf)?;
        Ok(())
    }
}

impl Decoder for Disconnect {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(Disconnect {
            reason: Chat::decode_from_read(reader)?,
        })
    }
}
