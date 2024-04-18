use std::io::Result;

use crate::{
    io::prelude::{Buffer, Decoder, Encoder, VarIntRead, VarIntWrite},
    server::chat::{Chat, ChatNbtWrite},
};

#[derive(Debug)]
pub struct CombatDeath {
    player_id: i32,
    message: Chat,
}

impl Encoder for CombatDeath {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(self.player_id)?;
        buf.write_nbt_chat(&self.message)?;
        Ok(())
    }
}

impl Decoder for CombatDeath {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(CombatDeath {
            player_id: reader.read_var_i32()?,
            message: Chat::decode_from_read(reader)?,
        })
    }
}
