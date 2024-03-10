use std::io::{prelude::Write, Result};

use uuid::Uuid;

use crate::{
    io::prelude::{Decoder, Encoder, Identifier, U8Read, U8Write, VarIntRead, VarIntWrite},
    protocol::v1_20_4::configuration::registry::Particle,
    server::{
        coordinates::{Direction, Position},
        prelude::Chat,
        slot::Slot,
    },
};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub struct SetEntityMetadata {
    pub entity_id: i32,
    pub metadata: EntityMeta,
}

impl Decoder for SetEntityMetadata {
    fn decode_from_read<R: std::io::prelude::Read>(reader: &mut R) -> Result<Self> {
        Ok(SetEntityMetadata {
            entity_id: reader.read_var_i32()?,
            metadata: EntityMeta::decode_from_read(reader)?,
        })
    }
}

impl Encoder for SetEntityMetadata {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_var_i32(self.entity_id)?;
        Ok(())
    }
}

pub struct EntityMeta {
    index: u8,
    data: Box<dyn EntityMetadata>,
}

//impl<Value: Encoder + Decoder> Decoder for EntityMeta<Value> {
//    fn decode_from_read<R: std::io::prelude::Read>(reader: &mut R) -> Result<Self> {
//        Ok(EntityMeta {
//            index: reader.read_u8()?,
//            data: todo!(),
//        })
//    }
//}
//
//impl<Value: Encoder + Decoder> Encoder for EntityMeta<Value> {
//    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
//        writer.write_u8(self.index)?;
//        Ok(())
//    }
//}

pub trait EntityMetadata {
    fn get_data_type_id() -> i32;
}

pub mod entity_meta_data_types {
    use std::io::{prelude::Write, Result};

    use uuid::Uuid;

    use crate::io::prelude::{Decoder, Encoder, Identifier, U8Read, U8Write, U8};

    use super::EntityMetadata;

    #[derive(derive_more::Deref)]
    pub struct Byte(U8);

    impl EntityMetadata for Byte {
        fn get_data_type_id() -> i32 {
            0
        }
    }
    impl Encoder for Byte {
        fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
            writer.write_u8(self.0)?;
            Ok(())
        }
    }

    impl Decoder for Byte {
        fn decode_from_read<R: std::io::prelude::Read>(reader: &mut R) -> Result<Self> {
            reader.read_u8()
        }
    }

    pub struct VarInt(crate::io::prelude::VarInt);

    impl EntityMetadata for VarInt {
        fn get_data_type_id() -> i32 {
            1
        }
    }
    pub struct VarLong(i64);
    pub struct Float(f32);
    pub struct String(std::string::String);
    pub struct Chat(crate::server::prelude::Chat);
    pub struct OptionalChat(Option<Chat>);
    pub struct Slot(crate::server::prelude::Slot);
    pub struct Boolean(bool);
    pub struct Rotation(f32, f32, f32);
    pub struct Position(crate::server::prelude::Position);
    pub struct OptionalPosition(Option<crate::server::prelude::Position>);
    pub struct Direction(crate::server::prelude::Direction);
    pub struct OptionalUuid(Option<Uuid>);
    pub struct BlockState(i32);
    pub struct OptionalBlockState(i32);
    pub struct Nbt(nbt::Value);
    pub struct Particle(crate::server::prelude::Particle);
    pub struct VillagerData(i32, i32, i32);
    pub struct OptionalVarInt(i32);
    pub struct Pose(i32);
    pub struct CatVariant(i32);
    pub struct FrogVariant(i32);
    pub struct OptionalGlobalPosition(Option<(Identifier, crate::server::prelude::Position)>);
    pub struct PaintingVariant(i32);
    pub struct SnifferState(i32);
    pub struct Vector3(f32, f32, f32);
    pub struct Quaternion(f32, f32, f32, f32);
}

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
