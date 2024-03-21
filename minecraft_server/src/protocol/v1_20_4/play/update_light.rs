use std::io::Result;

use crate::{
    io::prelude::{Buffer, Encoder, VarIntWrite},
    server::light::Light,
};

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
