use std::io::Result;

use crate::{
    io::prelude::{
        BoolRead, Buffer, Decoder, DecoderDeref, Encoder, F64Read, OptionRead, OptionWrite,
        VarIntRead, VarIntWrite, WriteBool,
    },
    server::prelude::DoublePosition,
};

pub struct DamageEvent {
    entity_id: i32,
    source_type_id: i32,
    source_cause_id: i32,
    source_direct_id: i32,
    source_position: Option<DoublePosition>,
}

impl Encoder for DamageEvent {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(self.entity_id)?;
        buf.write_var_i32(self.source_type_id)?;
        buf.write_var_i32(self.source_cause_id)?;
        buf.write_var_i32(self.source_direct_id)?;
        buf.write_option(&self.source_position)?;
        Ok(())
    }
}

impl Decoder for DamageEvent {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        impl !DecoderDeref for f64 {}
        impl Decoder for f64 {
            fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
                Ok(reader.read_f64()?)
            }
        }
        Ok(DamageEvent {
            entity_id: reader.read_var_i32()?,
            source_type_id: reader.read_var_i32()?,
            source_cause_id: reader.read_var_i32()?,
            source_direct_id: reader.read_var_i32()?,
            source_position: reader.read_option()?,
        })
    }
}
