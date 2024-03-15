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
#[derive(Debug, Clone)]
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
        use EntityMetadataValue::*;
        match self {
            Byte(_) => 0,
            VarInt(_) => 1,
            VarLong(_) => 2,
            Float(_) => 3,
            String(_) => 4,
            Chat(_) => 5,
            OptionalChat(_) => 6,
            Slot(_) => 7,
            Boolean(_) => 8,
            Rotation(_) => 9,
            Position(_) => 10,
            OptionalPosition(_) => 11,
            Direction(_) => 12,
            OptionalUuid(_) => 13,
            BlockState(_) => 14,
            OptionalBlockState(_) => 15,
            Nbt(_) => 16,
            Particle(_) => 17,
            VillagerData(_, _, _) => 18,
            OptionalVarInt(_) => 19,
            Pose(_) => 20,
            CatVariant(_) => 21,
            FrogVariant(_) => 22,
            OptionalGlobalPosition(_) => 23,
            PaintingVariant(_) => 24,
            SnifferState(_) => 25,
            Vector3(_, _, _) => 26,
            Quaternion(_) => 27,
        }
    }
}

impl Encoder for EntityMetadataValue {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        use EntityMetadataValue::*;
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
                //value.encode_to_buffer(buf)?;
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
