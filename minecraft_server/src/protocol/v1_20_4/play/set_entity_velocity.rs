use std::io::Result;

use crate::io::prelude::{Buffer, Decoder, Encoder, I16Read, I16Write, VarIntRead, VarIntWrite};

pub struct SetEntityVelocity {
    entity_id: i32,
    velocity_x: i16,
    velocity_y: i16,
    velocity_z: i16,
}

impl Encoder for SetEntityVelocity {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(self.entity_id)?;
        buf.write_i16(self.velocity_x)?;
        buf.write_i16(self.velocity_y)?;
        buf.write_i16(self.velocity_z)?;
        Ok(())
    }
}

impl Decoder for SetEntityVelocity {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(SetEntityVelocity {
            entity_id: reader.read_var_i32()?,
            velocity_x: reader.read_i16()?,
            velocity_y: reader.read_i16()?,
            velocity_z: reader.read_i16()?,
        })
    }
}
