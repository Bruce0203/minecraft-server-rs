use std::io::{Result, Write};

use uuid::Uuid;

use crate::{
    io::prelude::{
        Decoder, Encoder, F32Write, I64Write, Identifier, U8Read, U8Write, VarInt, VarIntWrite,
        VarString, VarStringWrite, F32, I64, U8,
    },
    server::{
        chat::ChatNbtWrite,
        prelude::{Chat, Direction, Particle, Position, Slot},
    },
};

pub type EntityMeta = Vec<EntityMetadataValue>;

impl Decoder for EntityMeta {
    fn decode_from_read<R: std::io::prelude::Read>(reader: &mut R) -> Result<Self> {
        todo!()
    }
}

impl Encoder for EntityMeta {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        for data in self.iter() {
            match data {
                EntityMetadataValue::Byte(value) => {
                    writer.write_var_i32(0)?;
                    writer.write_u8(**value)?
                }
                EntityMetadataValue::VarInt(value) => {
                    writer.write_var_i32(1)?;
                    writer.write_var_i32(**value)?;
                }
                EntityMetadataValue::VarLong(value) => {
                    writer.write_var_i32(2)?;
                    writer.write_i64(**value)?;
                }
                EntityMetadataValue::Float(value) => {
                    writer.write_var_i32(2)?;
                    writer.write_f32(**value)?;
                }
                EntityMetadataValue::String(value) => {
                    writer.write_var_i32(3)?;
                    writer.write_var_string(value)?;
                }
                EntityMetadataValue::Chat(value) => {
                    writer.write_var_i32(4)?;
                    writer.write_nbt_chat(&value)?;
                }
                EntityMetadataValue::OptionalChat(_) => todo!(),
                EntityMetadataValue::Slot(_) => todo!(),
                EntityMetadataValue::Boolean(_) => todo!(),
                EntityMetadataValue::Rotation(_, _, _) => todo!(),
                EntityMetadataValue::Position(_) => todo!(),
                EntityMetadataValue::OptionalPosition(_) => todo!(),
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
                EntityMetadataValue::Quaternion(_, _, _, _) => todo!(),
            }
        }
        writer.write_u8(0xff)?;
        Ok(())
    }
}

#[repr(i32)]
#[derive(Clone)]
pub enum EntityMetadataValue {
    Byte(U8) = 0,
    VarInt(VarInt) = 1,
    VarLong(I64) = 2,
    Float(F32) = 3,
    String(VarString) = 4,
    Chat(Chat) = 5,
    OptionalChat(Option<Chat>) = 6,
    Slot(Slot) = 7,
    Boolean(bool) = 8,
    Rotation(f32, f32, f32) = 9,
    Position(Position) = 10,
    OptionalPosition(Option<Position>) = 11,
    Direction(Direction) = 12,
    OptionalUuid(Option<Uuid>) = 13,
    BlockState(i32) = 14,
    OptionalBlockState(i32) = 15,
    Nbt(nbt::Value) = 16,
    Particle(Particle) = 17,
    VillagerData(i32, i32, i32) = 18,
    OptionalVarInt(i32) = 19,
    Pose(i32) = 20,
    CatVariant(i32) = 21,
    FrogVariant(i32) = 22,
    OptionalGlobalPosition(Option<(Identifier, Position)>) = 23,
    PaintingVariant(i32) = 24,
    SnifferState(i32) = 25,
    Vector3(f32, f32, f32) = 26,
    Quaternion(f32, f32, f32, f32) = 27,
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum EntityMetadataType {
    Byte = 0,
    VarInt = 1,
    VarLong = 2,
    Float = 3,
    String = 4,
    Chat = 5,
    OptionalChat = 6,
    Slot = 7,
    Boolean = 8,
    Rotation = 9,
    Position = 10,
    OptionalPosition = 11,
    Direction = 12,
    OptionalUuid = 13,
    BlockState = 14,
    OptionalBlockState = 15,
    Nbt = 16,
    Particle = 17,
    VillagerData = 18,
    OptionalVarInt = 19,
    Pose = 20,
    CatVariant = 21,
    FrogVariant = 22,
    OptionalGlobalPosition = 23,
    PaintingVariant = 24,
    SnifferState = 25,
    Vector3 = 26,
    Quaternion = 27,
}
