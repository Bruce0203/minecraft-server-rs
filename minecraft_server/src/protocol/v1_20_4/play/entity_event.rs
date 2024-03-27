use std::io::Result;

use crate::io::prelude::{Buffer, Decoder, Encoder, VarIntRead};

#[derive(Debug)]
pub struct EntityEvent {
    pub entity_id: i32,
    //TODO entity status
}

impl Encoder for EntityEvent {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        Ok(())
    }
}

impl Decoder for EntityEvent {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(EntityEvent {
            entity_id: reader.read_var_i32()?,
        })
    }
}
