use std::io::Result;

use crate::{
    io::prelude::{
        BoolRead, Buffer, Decoder, Encoder, I16Read, I16Write, VarIntRead, VarIntWrite, WriteBool,
    },
    server::prelude::Angle,
};

#[derive(Debug)]
pub struct UpdateEntityPositionAndRotation {
    entity_id: i32,
    delta_x: i16,
    delta_y: i16,
    delta_z: i16,
    yaw: Angle,
    pitch: Angle,
    on_ground: bool,
}

impl Encoder for UpdateEntityPositionAndRotation {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(self.entity_id)?;
        buf.write_i16(self.delta_x)?;
        buf.write_i16(self.delta_y)?;
        buf.write_i16(self.delta_z)?;
        self.yaw.encode_to_buffer(buf)?;
        self.pitch.encode_to_buffer(buf)?;
        buf.write_bool(self.on_ground)?;
        Ok(())
    }
}

impl Decoder for UpdateEntityPositionAndRotation {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(UpdateEntityPositionAndRotation {
            entity_id: reader.read_var_i32()?,
            delta_x: reader.read_i16()?,
            delta_y: reader.read_i16()?,
            delta_z: reader.read_i16()?,
            yaw: Angle::decode_from_read(reader)?,
            pitch: Angle::decode_from_read(reader)?,
            on_ground: reader.read_bool()?,
        })
    }
}
