use std::io::Result;

use crate::io::prelude::{Buffer, Decoder, Encoder, F32Read, F32Write, VarIntRead, VarIntWrite};

pub struct HurtAnimation {
    entity_id: i32,
    yaw: f32,
}

impl Encoder for HurtAnimation {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(self.entity_id)?;
        buf.write_f32(self.yaw)?;
        Ok(())
    }
}

impl Decoder for HurtAnimation {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(HurtAnimation {
            entity_id: reader.read_var_i32()?,
            yaw: reader.read_f32()?,
        })
    }
}
