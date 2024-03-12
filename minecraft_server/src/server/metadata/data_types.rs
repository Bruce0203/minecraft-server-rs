use std::{
    io::{prelude::Write, Result},
    panic::Location,
};

use bitflags::bitflags;
use delegate::delegate;
use uuid::Uuid;

use crate::{
    io::prelude::{Encoder, F32Write, Identifier, VarInt, VarIntWrite, VarString},
    server::prelude::{Chat, Direction, FloatRotation, Position, Slot},
};
use derive_more::{Deref, From, Into};

use super::prelude::EntityMeta;

macro_rules! enitty_metadata_type_id {
    ($(($index:expr, $type:ty), )*) => {
       $(
           impl EntityMeta for $type {
               fn get_metadata_type_id(&self) -> i32 {
                   $index
               }
           }
        )*
    };
}
enitty_metadata_type_id!(
    (0, u8),
    (1, VarInt),
    (2, i64),
    (3, f32),
    (4, VarString<32767>),
    (5, Chat),
    (6, Option<Chat>),
    (7, Slot),
    (8, bool),
    (9, FloatRotation),
    (10, Position),
    (11, Option<Position>),
    (12, Direction),
    (13, Option<Uuid>),
    //(14, i32/*BlockState*/),
);

#[derive(Deref)]
pub struct OptionalUuid(Option<Uuid>);
#[derive(Deref)]
pub struct BlockState(VarInt);
#[derive(Deref)]
pub struct OptionalBlockState(VarInt);
#[derive(Deref)]
pub struct Nbt(nbt::Value);
#[derive(Deref)]
pub struct Particle(crate::server::prelude::Particle);

pub struct VillagerData(VarInt, VarInt, VarInt);

impl Encoder for VillagerData {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        buf.write_var_i32(*self.0)?;
        buf.write_var_i32(*self.1)?;
        buf.write_var_i32(*self.2)?;
        Ok(())
    }
}

#[derive(Deref)]
pub struct OptionalVarInt(VarInt);
#[derive(Deref)]
pub struct Pose(VarInt);
#[derive(Deref)]
pub struct CatVariant(VarInt);
#[derive(Deref)]
pub struct FrogVariant(VarInt);
#[derive(Deref)]
pub struct OptionalGlobalPosition(Option<(Identifier, crate::server::prelude::Position)>);
#[derive(Deref)]
pub struct PaintingVariant(VarInt);
#[derive(Deref)]
pub struct SnifferState(VarInt);



#[repr(i32)]
pub enum EntityMetadataValue {
    Byte(u8) = 0,
    VarInt(i32) = 1,
    VarLong(i64) = 2,
    Float(f32) = 3,
    String(String) = 4,
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

