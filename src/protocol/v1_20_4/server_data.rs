use std::io::Cursor;

use crate::io::{
    encoding::Encoder, primitives::WriteBool, var_int::VarIntWrite, var_string::VarStringWrite, nbt::NbtNetworkWrite,
};

use crate::{
    protocol::prelude::PacketWriter,
    server::{chat::ChatNbtWrite, prelude::Chat},
};

pub struct ServerData {
    pub message_of_the_day: Chat,
    pub icon: Option<Vec<u8>>,
    pub enforce_secure_chat: bool,
}

impl Encoder for ServerData {
    fn encode_to_write<W: std::io::prelude::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_network_nbt(&self.message_of_the_day)?;
        if let Some(icon) = &self.icon {
            writer.write_bool(true)?;
            writer.write_var_i32(icon.len() as i32)?;
            writer.write_all(icon)?;
        } else {
            writer.write_bool(false)?;
        }
        writer.write_bool(self.enforce_secure_chat)?;
        Ok(())
    }
}

impl PacketWriter for ServerData {
    fn get_packet_id(&self, player: &mut crate::server::prelude::Player) -> std::io::Result<i32> {
        Ok(0x49)
    }
}
