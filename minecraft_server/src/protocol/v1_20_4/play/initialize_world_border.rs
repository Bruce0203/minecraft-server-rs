use std::io::Result;

use crate::io::prelude::{
    Buffer, Decoder, Encoder, F64Read, F64Write, I64Read, I64Write, VarIntRead, VarIntWrite,
    VarLongRead,
};

#[derive(Debug)]
pub struct InitializeWorldBorder {
    x: f64,
    z: f64,
    old_diameter: f64,
    new_diameter: f64,
    speed: i64,
    portal_teleport_boundary: i32,
    warning_blocks: i32,
    warning_time: i32,
}

impl Encoder for InitializeWorldBorder {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_f64(self.x)?;
        buf.write_f64(self.z)?;
        buf.write_f64(self.old_diameter)?;
        buf.write_f64(self.new_diameter)?;
        buf.write_i64(self.speed)?;
        buf.write_var_i32(self.portal_teleport_boundary)?;
        buf.write_var_i32(self.warning_blocks)?;
        buf.write_var_i32(self.warning_time)?;
        Ok(())
    }
}

impl Decoder for InitializeWorldBorder {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(InitializeWorldBorder {
            x: reader.read_f64()?,
            z: reader.read_f64()?,
            old_diameter: reader.read_f64()?,
            new_diameter: reader.read_f64()?,
            speed: reader.read_var_long()?,
            portal_teleport_boundary: reader.read_var_i32()?,
            warning_blocks: reader.read_var_i32()?,
            warning_time: reader.read_var_i32()?,
        })
    }
}
