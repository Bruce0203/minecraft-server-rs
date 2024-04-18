use std::io::Result;

use crate::{
    io::prelude::{Buffer, Decoder, Encoder, VarIntRead, VarIntWrite},
    server::light::Light,
};

#[derive(Debug)]
pub struct UpdateLight {
    pub x: i32,
    pub z: i32,
    pub light: Light,
}

impl Encoder for UpdateLight {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(self.x)?;
        buf.write_var_i32(self.z)?;
        self.light.encode_to_buffer(buf)?;
        Ok(())
    }
}

impl Decoder for UpdateLight {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(UpdateLight {
            x: reader.read_var_i32()?,
            z: reader.read_var_i32()?,
            light: Light::decode_from_read(reader)?,
        })
    }
}
