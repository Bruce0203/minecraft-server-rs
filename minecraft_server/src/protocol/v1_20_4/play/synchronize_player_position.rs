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
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        self.location.encode_to_buffer(buf)?;
        buf.write_u8(self.flags)?;
        buf.write_var_i32(self.teleport_id)?;
        Ok(())
    }
}
