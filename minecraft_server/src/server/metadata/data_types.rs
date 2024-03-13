use std::{
    io::{prelude::Write, Result},
    ops::Deref,
    panic::Location,
};

use bitflags::bitflags;
use delegate::delegate;
use uuid::Uuid;

use crate::{
    io::prelude::{
        Buffer, Encoder, F32Write, Identifier, OptionWrite, U8Write, VarIntWrite, VarLongWrite,
        VarString,
    },
    server::prelude::{Chat, FloatRotation, Quaternion},
};

#[repr(i32)]
#[derive(Clone)]
pub enum EntityMetadataValue {
    Byte(u8) = 0,
    VarInt(i32) = 1,
    VarLong(i64) = 2,
    Float(f32) = 3,
    String(VarString<32767>) = 4,
    Chat(Box<crate::server::prelude::Chat>) = 5,
    OptionalChat(Option<Box<crate::server::prelude::Chat>>) = 6,
    Slot(crate::server::prelude::Slot) = 7,
    Boolean(bool) = 8,
    Rotation(FloatRotation) = 9,
    Position(crate::server::prelude::Position) = 10,
    OptionalPosition(Option<crate::server::prelude::Position>) = 11,
    Direction(crate::server::prelude::Direction) = 12,
    OptionalUuid(Option<Uuid>) = 13,
    BlockState(i32) = 14,
    OptionalBlockState(i32) = 15,
    Nbt(nbt::Value) = 16,
    Particle(Box<crate::server::prelude::Particle>) = 17,
    VillagerData(i32, i32, i32) = 18,
    OptionalVarInt(i32) = 19,
    Pose(i32) = 20,
    CatVariant(i32) = 21,
    FrogVariant(i32) = 22,
    OptionalGlobalPosition(Option<(Identifier, crate::server::prelude::Position)>) = 23,
    PaintingVariant(i32) = 24,
    SnifferState(i32) = 25,
    Vector3(f32, f32, f32) = 26,
    Quaternion(crate::server::prelude::Quaternion) = 27,
}

impl EntityMetadataValue {
    pub const NONE: std::option::Option<EntityMetadataValue> = None;

    pub fn get_metadata_type_id(&self) -> i32 {
        match self {
            EntityMetadataValue::Byte(_) => 0,
            EntityMetadataValue::VarInt(_) => 1,
            EntityMetadataValue::VarLong(_) => 2,
            EntityMetadataValue::Float(_) => 3,
            EntityMetadataValue::String(_) => 4,
            EntityMetadataValue::Chat(_) => 5,
            EntityMetadataValue::OptionalChat(_) => 6,
            EntityMetadataValue::Slot(_) => 7,
            EntityMetadataValue::Boolean(_) => 8,
            EntityMetadataValue::Rotation(_) => 9,
            EntityMetadataValue::Position(_) => 10,
            EntityMetadataValue::OptionalPosition(_) => 11,
            EntityMetadataValue::Direction(_) => 12,
            EntityMetadataValue::OptionalUuid(_) => 13,
            EntityMetadataValue::BlockState(_) => 14,
            EntityMetadataValue::OptionalBlockState(_) => 15,
            EntityMetadataValue::Nbt(_) => 16,
            EntityMetadataValue::Particle(_) => 17,
            EntityMetadataValue::VillagerData(_, _, _) => 18,
            EntityMetadataValue::OptionalVarInt(_) => 19,
            EntityMetadataValue::Pose(_) => 20,
            EntityMetadataValue::CatVariant(_) => 21,
            EntityMetadataValue::FrogVariant(_) => 22,
            EntityMetadataValue::OptionalGlobalPosition(_) => 23,
            EntityMetadataValue::PaintingVariant(_) => 24,
            EntityMetadataValue::SnifferState(_) => 25,
            EntityMetadataValue::Vector3(_, _, _) => 26,
            EntityMetadataValue::Quaternion(_) => 27,
        }
    }
}

impl Encoder for EntityMetadataValue {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        match self {
            EntityMetadataValue::Byte(value) => {
                buf.write_u8(*value)?;
            }
            EntityMetadataValue::VarInt(value) => {
                buf.write_var_i32(*value)?;
            }
            EntityMetadataValue::VarLong(value) => {
                buf.write_var_long(*value)?;
            }
            EntityMetadataValue::Float(value) => {
                buf.write_f32(*value)?;
            }
            EntityMetadataValue::String(value) => {
                value.encode_to_buffer(buf)?;
            }
            EntityMetadataValue::Chat(value) => {
                value.encode_to_buffer(buf)?;
            }
            EntityMetadataValue::OptionalChat(value) => {
                todo!()
            }
            EntityMetadataValue::Slot(value) => {
                value.encode_to_buffer(buf)?;
            }
            EntityMetadataValue::Boolean(value) => {
                value.encode_to_buffer(buf)?;
            }
            EntityMetadataValue::Rotation(value) => {
                value.encode_to_buffer(buf)?;
            }
            EntityMetadataValue::Position(value) => {
                value.encode_to_buffer(buf)?;
            }
            EntityMetadataValue::OptionalPosition(_) => {}
            EntityMetadataValue::Direction(_) => todo!(),
            EntityMetadataValue::OptionalUuid(_) => todo!(),
            EntityMetadataValue::BlockState(_) => todo!(),
            EntityMetadataValue::OptionalBlockState(_) => todo!(),
            EntityMetadataValue::Nbt(_) => todo!(),
            EntityMetadataValue::Particle(_) => todo!(),
            EntityMetadataValue::VillagerData(_, _, _) => todo!(),
            EntityMetadataValue::OptionalVarInt(_) => todo!(),
            EntityMetadataValue::Pose(_) => todo!(),
            EntityMetadataValue::CatVariant(_) => todo!(),
            EntityMetadataValue::FrogVariant(_) => todo!(),
            EntityMetadataValue::OptionalGlobalPosition(_) => todo!(),
            EntityMetadataValue::PaintingVariant(_) => todo!(),
            EntityMetadataValue::SnifferState(_) => todo!(),
            EntityMetadataValue::Vector3(_, _, _) => todo!(),
            EntityMetadataValue::Quaternion(_) => todo!(),
        }
        Ok(())
    }
}
