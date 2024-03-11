use std::io::{Result, Write};

use crate::{
    io::prelude::{Encoder, U8Write, VarIntWrite},
    net::prelude::{PacketId, Socket},
    server::{coordinates::Location, prelude::GamePlayer},
};

pub struct SyncPlayerPosition {
    pub location: Location,
    pub flags: u8,
    pub teleport_id: i32,
}

impl Encoder for SyncPlayerPosition {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.location.encode_to_write(writer)?;
        writer.write_u8(self.flags)?;
        writer.write_var_i32(self.teleport_id)?;
        Ok(())
    }
}

