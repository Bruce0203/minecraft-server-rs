use std::io::{prelude::Write, Result};

use crate::io::prelude::{Buffer, Decoder, Encoder, F32Read, F32Write};
use crate::net::prelude::{PacketId, Socket};
use crate::server::coordinates::Position;
use crate::server::prelude::GamePlayer;

pub struct SetDefaultPosition {
    pub location: Position,
    pub angle: f32,
}

impl Encoder for SetDefaultPosition {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        self.location.encode_to_buffer(buf)?;
        buf.write_f32(self.angle)?;
        Ok(())
    }
}

impl Decoder for SetDefaultPosition {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(SetDefaultPosition {
            location: Position::decode_from_read(reader)?,
            angle: reader.read_f32()?,
        })
    }
}
