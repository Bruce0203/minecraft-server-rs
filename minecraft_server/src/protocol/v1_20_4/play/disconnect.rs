use std::io::Result;

use crate::{
    io::prelude::{Buffer, Decoder, Encoder, ReadOnlyBuffer},
    server::chat::Chat,
};

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
    fn decode_from_read(reader: &mut ReadOnlyBuffer) -> Result<Self> {
        Ok(Disconnect {
            reason: Chat::decode_from_read(reader)?,
        })
    }
}
