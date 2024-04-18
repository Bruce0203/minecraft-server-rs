use std::{any::Any, fmt::Debug, io::Result, ops::Deref};

use bitflags::Flags;
use uuid::Uuid;

use crate::{
    io::prelude::{
        Buffer, Encoder, EncoderDeref, Identifier, U8Write, VarInt, VarIntWrite, VarString,
    },
    server::prelude::{
        Chat, Direction, FloatRotation, Particle, Pose, Position, Quaternion, Slot, Vector3,
    },
};
use derive_more::Deref;

#[derive(Debug)]
pub struct EntityMetadata(pub Box<dyn Encoder>);

impl Encoder for EntityMetadata {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        self.0.encode_to_buffer(buf)?;
        buf.write_u8(0xff)?;
        Ok(())
    }
}

pub trait MetadataType: Encoder {
    #[inline(always)]
    fn get_type_id() -> i32;
}

pub trait MetadataField: MetadataType {
    #[inline(always)]
    fn get_index(&self) -> u8;
}

#[derive(Debug)]
pub struct MetadataEncoder<E: MetadataType> {
    index: usize,
    metadata: E,
}

impl<E: MetadataType> MetadataField for MetadataEncoder<E> {
    #[inline(always)]
    fn get_index(&self) -> u8 {
        self.index as u8
    }
}

impl<E: MetadataType> MetadataType for MetadataEncoder<E> {
    fn get_type_id() -> i32 {
        E::get_type_id()
    }
}

impl<E: Encoder + MetadataType> !EncoderDeref for MetadataEncoder<E> {}

impl<E: Encoder + MetadataType> Encoder for MetadataEncoder<E> {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_u8(self.index as u8)?;
        buf.write_var_i32(E::get_type_id())?;
        E::encode_to_buffer(&self.metadata, buf)?;
        Ok(())
    }
}

macro_rules! metadata_types {
    ($(($typ:ty, $id:expr),)*) => {
        $(
        impl MetadataType for $typ {
            fn get_type_id() -> i32 {
                $id
            }
        }
        )*
    };
}

metadata_types!(
    (u8, 0),
    (VarInt, 1),
    (i64, 2),
    (f32, 3),
    (VarString<32767>, 4),
    (Chat, 5),
    (Option<Chat>, 6),
    (Slot, 7),
    (bool, 8),
    (FloatRotation, 9),
    (Position, 10),
    (Option<Position>, 11),
    (Direction, 12),
    (Option<Uuid>, 13),
    (BlockState, 14),
    (Option<BlockState>, 15),
    //(nbt::Value, 16),
    (Particle, 17),
    (VillagerData, 18),
    (Option<VarInt>, 19),
    (Pose, 20),
    (CatVariant, 21),
    (FrogVariant, 22),
    (Option<(Identifier, Position)>, 23),
    (PaintingVariant, 24),
    (SnifferState, 25),
    (Vector3, 26),
    (Quaternion, 27),
);

impl Encoder for (Identifier, Position) {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        todo!()
    }
}

#[derive(Debug, Deref, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct BlockState(VarInt);

impl Encoder for BlockState {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct VillagerData {
    villager_type: VillagerType,
    villager_profession: VillagerProfession,
    level: i32,
}

impl Encoder for VillagerData {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        todo!()
    }
}

#[derive(Debug)]
#[repr(i32)]
pub enum VillagerType {
    Desert = 0,
    Jungle = 1,
    Plains = 2,
    Savanna = 3,
    Snow = 4,
    Swamp = 5,
    Taiga = 6,
}

#[derive(Debug)]
#[repr(i32)]
pub enum VillagerProfession {
    None = 0,
    Armorer = 1,
    Butcher = 2,
    Cartographer = 3,
    Cleric = 4,
    Farmer = 5,
    Fisherman = 6,
    Fletcher = 7,
    LeatherWorker = 8,
    Librarian = 9,
    Mason = 10,
    Nitwit = 11,
    Shepherd = 12,
    Toolsmith = 13,
    WeaponSmith = 14,
}

#[derive(Debug)]
pub enum CatVariant {}

impl Encoder for CatVariant {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        todo!()
    }
}

#[derive(Debug)]
pub enum FrogVariant {}

impl Encoder for FrogVariant {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        todo!()
    }
}

#[derive(Debug)]
pub enum PaintingVariant {}

impl Encoder for PaintingVariant {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        todo!()
    }
}

#[derive(Debug)]
pub enum SnifferState {}

impl Encoder for SnifferState {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        todo!()
    }
}
