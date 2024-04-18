use std::io::Result;

use crate::{
    io::prelude::{
        BoolRead, Buffer, Decoder, Encoder, F64Read, F64Write, VarIntRead, VarIntWrite, WriteBool,
    },
    server::prelude::Angle,
};

#[derive(Debug)]
pub struct TeleportEntity {
    entity_id: i32,
    x: f64,
    y: f64,
    z: f64,
    yaw: Angle,
    pitch: Angle,
    on_ground: bool,
}

impl Encoder for TeleportEntity {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(self.entity_id);
        buf.write_f64(self.x)?;
        buf.write_f64(self.y)?;
        buf.write_f64(self.z)?;
        self.yaw.encode_to_buffer(buf)?;
        self.pitch.encode_to_buffer(buf)?;
        buf.write_bool(self.on_ground)?;
        Ok(())
    }
}

impl Decoder for TeleportEntity {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(TeleportEntity {
            entity_id: reader.read_var_i32()?,
            x: reader.read_f64()?,
            y: reader.read_f64()?,
            z: reader.read_f64()?,
            yaw: Angle::decode_from_read(reader)?,
            pitch: Angle::decode_from_read(reader)?,
            on_ground: reader.read_bool()?,
        })
    }
}
