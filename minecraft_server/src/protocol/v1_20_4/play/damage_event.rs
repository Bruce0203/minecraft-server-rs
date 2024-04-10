use std::io::Result;

use crate::io::prelude::{
    BoolRead, Buffer, Decoder, DecoderDeref, Encoder, F64Read, OptionRead, OptionWrite, VarIntRead,
    VarIntWrite, WriteBool,
};

pub struct DamageEvent {
    entity_id: i32,
    source_type_id: i32,
    source_cause_id: i32,
    source_direct_id: i32,
    has_source_position: bool,
    source_position_x: Option<f64>,
    source_position_y: Option<f64>,
    source_position_z: Option<f64>,
}

impl Encoder for DamageEvent {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(self.entity_id)?;
        buf.write_var_i32(self.source_type_id)?;
        buf.write_var_i32(self.source_cause_id)?;
        buf.write_var_i32(self.source_direct_id)?;
        buf.write_bool(self.has_source_position)?;
        buf.write_option(&self.source_position_x)?;
        buf.write_option(&self.source_position_y)?;
        buf.write_option(&self.source_position_z)?;
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
            has_source_position: reader.read_bool()?,
            source_position_x: reader.read_option()?,
            source_position_y: reader.read_option()?,
            source_position_z: reader.read_option()?,
        })
    }
}
