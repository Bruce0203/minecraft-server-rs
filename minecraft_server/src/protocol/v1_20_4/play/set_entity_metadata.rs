use uuid::Uuid;

use crate::{
    io::prelude::Identifier,
    protocol::v1_20_4::configuration::registry::Particle,
    server::{coordinates::Position, prelude::Chat, slot::Slot},
};

pub struct SetEntityMetadata {
    pub entity_id: i32,
    pub metadata: EntityMetadata,
}

#[repr(i32)]
pub enum EntityMetadata {
    Byte(u8),
    VarInt(i32),
    VarLong(i64),
    Float(f32),
    String(String),
    Chat(Chat),
    Slot(Slot),
    Boolean(bool),
    Rotation(f32, f32, f32),
    Position(Position),
    OptionalPosition(Option<Position>),
    Direction(Direction),
    OptionalUuid(Option<Uuid>),
    BlockState(i32),
    OptionalBlockState(i32),
    Nbt(nbt::Value),
    Particle(Particle),
    VillagerData(i32, i32, i32),
    OptionalVarInt(i32),
    Pose(i32),
    CatVariant(i32),
    FrogVariant(i32),
    OptionalGlobalPosition(Option<(Option<Identifier>, Option<Position>)>),
    PaintingVariant(i32),
    SnifferState(i32),
    Vector3(f32, f32, f32),
    Quaternion(f32, f32, f32, f32),
}

#[repr(i32)]
pub enum Direction {
    Down = 0,
    Up = 1,
    North = 2,
    South = 3,
    West = 4,
    East = 5,
}
