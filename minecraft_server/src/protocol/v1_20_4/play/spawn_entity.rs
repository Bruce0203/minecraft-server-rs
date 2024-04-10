use std::io::Result;

use uuid::Uuid;

use crate::{
    io::prelude::{Buffer, Decoder, Encoder, F64Read, I16Read, UuidRead, VarIntRead},
    server::prelude::Angle,
};

pub struct SpawnEntity {
    entity_id: i32,
    entity_uuid: Uuid,
    entity_type: i32,
    x: f64,
    y: f64,
    z: f64,
    pitch: Angle,
    yaw: Angle,
    head_yaw: Angle,
    data: i32,
    velocity_x: i16,
    velocity_y: i16,
    velocity_z: i16,
}

impl Encoder for SpawnEntity {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        todo!()
    }
}

impl Decoder for SpawnEntity {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(SpawnEntity {
            entity_id: reader.read_var_i32()?,
            entity_uuid: reader.read_uuid()?,
            entity_type: reader.read_var_i32()?,
            x: reader.read_f64()?,
            y: reader.read_f64()?,
            z: reader.read_f64()?,
            pitch: Angle::decode_from_read(reader)?,
            yaw: Angle::decode_from_read(reader)?,
            head_yaw: Angle::decode_from_read(reader)?,
            data: reader.read_var_i32()?,
            velocity_x: reader.read_i16()?,
            velocity_y: reader.read_i16()?,
            velocity_z: reader.read_i16()?,
        })
    }
}
