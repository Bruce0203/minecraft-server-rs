use std::io::Result;

use crate::io::prelude::{
    BoolRead, Buffer, Decoder, Encoder, I16Read, I16Write, VarIntRead, VarIntWrite, WriteBool,
};

pub struct UpdateEntityPosition {
    entity_id: i32,
    delta_x: i16,
    delta_y: i16,
    delta_z: i16,
    on_ground: bool,
}

impl Encoder for UpdateEntityPosition {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(self.entity_id)?;
        buf.write_i16(self.delta_x)?;
        buf.write_i16(self.delta_y)?;
        buf.write_i16(self.delta_z)?;
        buf.write_bool(self.on_ground)?;
        Ok(())
    }
}

impl Decoder for UpdateEntityPosition {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(UpdateEntityPosition {
            entity_id: reader.read_var_i32()?,
            delta_x: reader.read_i16()?,
            delta_y: reader.read_i16()?,
            delta_z: reader.read_i16()?,
            on_ground: reader.read_bool()?,
        })
    }
}
