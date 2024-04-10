use std::io::Result;

use crate::io::prelude::{
    Buffer, Decoder, DecoderDeref, Encoder, EncoderDeref, VarIntRead, VarIntSizedVecRead,
    VarIntSizedVecWrite, VarIntWrite,
};

pub struct RemoveEntities {
    entity_ids: Vec<i32>,
}

impl Encoder for RemoveEntities {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        impl !EncoderDeref for i32 {}
        impl Encoder for i32 {
            fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
                buf.write_var_i32(*self)?;
                Ok(())
            }
        }
        buf.write_var_int_sized_vec(&self.entity_ids)?;
        Ok(())
    }
}

impl Decoder for RemoveEntities {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        impl !DecoderDeref for i32 {}
        impl Decoder for i32 {
            fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
                Ok(reader.read_var_i32()?)
            }
        }
        Ok(RemoveEntities {
            entity_ids: reader.read_var_int_sized_vec()?,
        })
    }
}
