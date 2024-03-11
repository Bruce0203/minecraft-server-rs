use std::io::{Cursor, Write};

use crate::io::prelude::{Encoder, NbtNetworkWrite, VarIntWrite, WriteBool};
use crate::net::prelude::{PacketId, Socket};
use crate::server::prelude::{Chat, GamePlayer, GameServer};

pub struct ServerData {
    pub message_of_the_day: Chat,
    pub icon: Option<Vec<u8>>,
    pub enforce_secure_chat: bool,
}

impl Encoder for ServerData {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> std::io::Result<()> {
        buf.write_network_nbt(&self.message_of_the_day)?;
        if let Some(icon) = &self.icon {
            buf.write_bool(true)?;
            buf.write_var_i32(icon.len() as i32)?;
            buf.write_all(icon)?;
        } else {
            buf.write_bool(false)?;
        }
        buf.write_bool(self.enforce_secure_chat)?;
        Ok(())
    }
}
