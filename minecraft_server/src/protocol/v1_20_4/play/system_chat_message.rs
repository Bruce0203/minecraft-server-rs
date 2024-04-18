use std::io::Result;

use crate::{
    io::prelude::{BoolRead, Buffer, Decoder, Encoder, WriteBool},
    server::chat::{Chat, ChatNbtWrite},
};

#[derive(Debug)]
pub struct SystemChatMessage {
    pub content: Chat, //TODO Limit to 262144 bytes
    pub overlay: bool,
}

impl Encoder for SystemChatMessage {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_nbt_chat(&self.content)?;
        buf.write_bool(self.overlay)?;
        Ok(())
    }
}

impl Decoder for SystemChatMessage {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(SystemChatMessage {
            content: Chat::decode_from_read(reader)?,
            overlay: reader.read_bool()?,
        })
    }
}
