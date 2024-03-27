use std::io::{Cursor, Result, Write};

use crate::io::prelude::{
    BoolRead, Buffer, Decoder, DecoderDeref, Encoder, EncoderDeref, NbtNetworkWrite, OptionRead,
    U8Read, VarIntRead, VarIntSizedVecRead, VarIntWrite, WriteBool,
};
use crate::net::prelude::{PacketId, Socket};
use crate::server::prelude::{Chat, GamePlayer, GameServer};

#[derive(Debug)]
pub struct ServerData {
    pub message_of_the_day: Chat,
    pub icon: Option<Icon>,
    pub enforce_secure_chat: bool,
}

#[derive(Debug)]
pub struct Icon(pub Vec<u8>);

impl DecoderDeref for Icon {}
impl EncoderDeref for Icon {}

impl Decoder for Icon {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        let len = reader.read_var_i32()?;
        let mut icon = Vec::with_capacity(len as usize);
        for byte in 0..len {
            icon.push(reader.read_u8()?)
        }
        Ok(Icon(icon))
    }
}

impl Encoder for ServerData {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_network_nbt(&self.message_of_the_day)?;
        if let Some(icon) = &self.icon {
            buf.write_bool(true)?;
            buf.write_var_i32(icon.0.len() as i32)?;
            buf.write_all(&icon.0)?;
        } else {
            buf.write_bool(false)?;
        }
        buf.write_bool(self.enforce_secure_chat)?;
        Ok(())
    }
}

impl Decoder for ServerData {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(ServerData {
            message_of_the_day: Chat::decode_from_read(reader)?,
            icon: reader.read_option()?,
            enforce_secure_chat: reader.read_bool()?,
        })
    }
}
