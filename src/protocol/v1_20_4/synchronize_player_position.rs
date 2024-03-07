use crate::{
    io::prelude::{Encoder, U8Write, VarIntWrite},
    net::prelude::PacketIdentnifier,
    server::coordinates::Location,
};

pub struct SyncPlayerPosition {
    pub location: Location,
    pub flags: u8,
    pub teleport_id: i32,
}

impl Encoder for SyncPlayerPosition {
    fn encode_to_write<W: std::io::prelude::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.location.encode_to_write(writer)?;
        writer.write_u8(self.flags)?;
        writer.write_var_i32(self.teleport_id)?;
        Ok(())
    }
}

impl PacketIdentnifier for SyncPlayerPosition {
    fn get_packet_id(&self, player: &mut crate::net::prelude::Player) -> std::io::Result<i32> {
        Ok(0x3E)
    }
}
