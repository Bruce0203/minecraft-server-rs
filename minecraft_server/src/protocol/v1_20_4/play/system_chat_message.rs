use std::io::Result;

use crate::{
    io::prelude::{Buffer, Encoder, WriteBool},
    server::chat::{Chat, ChatNbtWrite},
};

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
