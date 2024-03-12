use std::io::Result;

use bitflags::bitflags;

use crate::{
    io::prelude::{Bool, Buffer, Encoder, VarInt},
    server::prelude::{Chat, Pose},
};

pub struct Entity {
    entity_byte: EntityByte,
    air_ticks: VarInt,
    custom_name: Option<Chat>,
    is_custom_name_visible: Bool,
    is_silent: Bool,
    has_no_gravity: Bool,
    pose: Pose,
    tick_frozen_in_powdered_snow: VarInt,
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            entity_byte: EntityByte::None,
            air_ticks: 0.into(),
            custom_name: None,
            is_custom_name_visible: false.into(),
            is_silent: false.into(),
            has_no_gravity: false.into(),
            pose: Pose::default(),
            tick_frozen_in_powdered_snow: 0.into(),
        }
    }
}

bitflags! {
    pub struct EntityByte: u8 {
        const IsOnFire = 0x01;
        const IsCrouching = 0x02;
        const IsSprinting = 0x08;
        const IsSwimming = 0x10;
        const IsInvisible = 0x20;
        const HasGlowingEffect = 0x40;
        const IsFlyingWithAnElytra = 0x80;
        const None = 0;
    }
}

impl Encoder for Entity {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        Ok(())
    }
}
