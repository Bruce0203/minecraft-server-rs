use std::io::Result;

use crate::{
    io::prelude::{Buffer, Decoder, Encoder, VarIntRead, VarIntWrite},
    server::prelude::Angle,
};

#[derive(Debug)]
pub struct SetHeadRotation {
    entity_id: i32,
    head_yaw: Angle,
}

impl Encoder for SetHeadRotation {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(self.entity_id)?;
        self.head_yaw.encode_to_buffer(buf)?;
        Ok(())
    }
}

impl Decoder for SetHeadRotation {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(SetHeadRotation {
            entity_id: reader.read_var_i32()?,
            head_yaw: Angle::decode_from_read(reader)?,
        })
    }
}
