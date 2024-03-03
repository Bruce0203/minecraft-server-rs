use mc_io::{
    encoding::Encoder, primitives::WriteBool, var_int::VarIntWrite, var_string::VarStringWrite,
};

use crate::connection::prelude::PacketWriter;

pub struct ServerData {
    pub message_of_the_day: String,
    pub icon: Option<Vec<u8>>,
    pub enforce_secure_chat: bool,
}

impl Encoder for ServerData {
    fn encode_to_write<W: std::io::prelude::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_var_string(self.message_of_the_day.as_str())?;
        if let Some(icon) = &self.icon {
            writer.write_bool(true)?;
            writer.write_var_i32(icon.len() as i32)?;
            writer.write_all(icon)?;
        } else {
            writer.write_bool(false)?;
        }
        Ok(())
    }
}

impl PacketWriter for ServerData {
    fn get_packet_id(&self, player: &mut crate::server::prelude::Player) -> std::io::Result<i32> {
        Ok(0x49)
    }
}
