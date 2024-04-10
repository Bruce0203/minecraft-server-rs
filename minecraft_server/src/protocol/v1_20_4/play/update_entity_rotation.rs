use std::io::Result;

use crate::{
    io::prelude::{
        BoolRead, Buffer, Decoder, Encoder, I16Read, VarIntRead, VarIntWrite, WriteBool,
    },
    server::prelude::Angle,
};

pub struct UpdateEntityRotation {
    entity_id: i32,
    yaw: Angle,
    pitch: Angle,
    on_ground: bool,
}

impl Encoder for UpdateEntityRotation {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(self.entity_id)?;
        self.yaw.encode_to_buffer(buf)?;
        self.pitch.encode_to_buffer(buf)?;
        buf.write_bool(self.on_ground)?;
        Ok(())
    }
}

impl Decoder for UpdateEntityRotation {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(UpdateEntityRotation {
            entity_id: reader.read_var_i32()?,
            yaw: Angle::decode_from_read(reader)?,
            pitch: Angle::decode_from_read(reader)?,
            on_ground: reader.read_bool()?,
        })
    }
}
