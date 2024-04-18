use std::io::Result;

use crate::io::prelude::{Buffer, Decoder, Encoder, VarIntRead, VarIntWrite};

#[derive(Debug)]
pub struct ConfirmTeleportation {
    pub teleport_id: i32,
}

impl Encoder for ConfirmTeleportation {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(self.teleport_id)?;
        Ok(())
    }
}

impl Decoder for ConfirmTeleportation {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(ConfirmTeleportation {
            teleport_id: reader.read_var_i32()?,
        })
    }
}
