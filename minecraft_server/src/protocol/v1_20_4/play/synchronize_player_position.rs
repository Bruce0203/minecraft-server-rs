use std::io::{Result, Write};

use crate::{
    io::prelude::{Buffer, Decoder, Encoder, U8Read, U8Write, VarIntRead, VarIntWrite},
    net::prelude::{PacketId, Socket},
    server::{coordinates::Location, prelude::GamePlayer},
};

#[derive(Debug)]
pub struct SyncPlayerPosition {
    pub location: Location,
    pub flags: u8,
    pub teleport_id: i32,
}

impl Encoder for SyncPlayerPosition {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        self.location.encode_to_buffer(buf)?;
        buf.write_u8(self.flags)?;
        buf.write_var_i32(self.teleport_id)?;
        Ok(())
    }
}

impl Decoder for SyncPlayerPosition {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(SyncPlayerPosition {
            location: Location::decode_from_read(reader)?,
            flags: reader.read_u8()?,
            teleport_id: reader.read_var_i32()?,
        })
    }
}
