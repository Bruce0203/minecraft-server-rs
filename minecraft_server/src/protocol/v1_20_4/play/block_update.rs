use std::io::Result;

use crate::{
    io::prelude::{Buffer, Decoder, Encoder, VarIntRead, VarIntWrite},
    server::prelude::{Location, Position},
};

pub struct BlockUpdate {
    position: Position,
    block_id: i32,
}

impl Encoder for BlockUpdate {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        self.position.encode_to_buffer(buf)?;
        buf.write_var_i32(self.block_id)?;
        Ok(())
    }
}

impl Decoder for BlockUpdate {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(BlockUpdate {
            position: Position::decode_from_read(reader)?,
            block_id: reader.read_var_i32()?,
        })
    }
}
