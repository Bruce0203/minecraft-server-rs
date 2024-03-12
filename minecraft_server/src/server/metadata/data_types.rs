use std::{
    io::{prelude::Write, Result},
    panic::Location,
};

use bitflags::bitflags;
use delegate::delegate;
use uuid::Uuid;

use crate::io::prelude::{
    Bool, Encoder, F32Write, Identifier, VarInt, VarIntWrite, VarString, F32, I64,
};
use derive_more::{Deref, From, Into};

#[derive(Deref)]
pub struct VarLong(I64);
#[derive(Deref)]
pub struct Float(F32);
#[derive(Deref)]
pub struct String(VarString<32767>);
#[derive(Deref)]
pub struct Chat(crate::server::prelude::Chat);
#[derive(Deref)]
pub struct OptionalChat(Option<Chat>);
#[derive(Deref)]
pub struct Slot(crate::server::prelude::Slot);
#[derive(Deref)]
pub struct Boolean(Bool);

pub struct Rotation(F32, F32, F32);

impl Encoder for Rotation {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        buf.write_f32(*self.0)?;
        buf.write_f32(*self.1)?;
        buf.write_f32(*self.2)?;
        Ok(())
    }
}

#[derive(Deref)]
pub struct Position(crate::server::prelude::Position);
#[derive(Deref)]
pub struct OptionalPosition(Option<crate::server::prelude::Position>);
#[derive(Deref)]
pub struct Direction(crate::server::prelude::Direction);
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

pub struct Vector3(F32, F32, F32);

impl Encoder for Vector3 {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        buf.write_f32(*self.0)?;
        buf.write_f32(*self.1)?;
        buf.write_f32(*self.2)?;
        Ok(())
    }
}

pub struct Quaternion(F32, F32, F32, F32);

impl Encoder for Quaternion {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        buf.write_f32(*self.0)?;
        buf.write_f32(*self.1)?;
        buf.write_f32(*self.2)?;
        buf.write_f32(*self.3)?;
        Ok(())
    }
}

