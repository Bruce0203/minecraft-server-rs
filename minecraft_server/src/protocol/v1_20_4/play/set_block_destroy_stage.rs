use std::io::Result;

use crate::{
    io::prelude::{Buffer, Decoder, Encoder, U8Read, U8Write, VarIntRead, VarIntWrite},
    server::prelude::Position,
};

#[derive(Debug)]
pub struct SetBlockDestoryStage {
    entity_id: i32,
    location: Position,
    destory_stage: u8,
}

impl Encoder for SetBlockDestoryStage {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(self.entity_id)?;
        self.location.encode_to_buffer(buf)?;
        buf.write_u8(self.destory_stage)?;
        Ok(())
    }
}

impl Decoder for SetBlockDestoryStage {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(SetBlockDestoryStage {
            entity_id: reader.read_var_i32()?,
            location: Position::decode_from_read(reader)?,
            destory_stage: reader.read_u8()?,
        })
    }
}
