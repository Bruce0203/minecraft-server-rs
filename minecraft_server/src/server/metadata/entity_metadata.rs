use std::io::Result;

use crate::io::prelude::{Buffer, Encoder, Identifier, U8Write, VarInt, VarIntWrite};
use bitflags::bitflags;
use derive_more::{Deref, From, Into};
use dyn_clone::DynClone;
use uuid::Uuid;

use super::prelude::EntityMetadataValue;

#[derive(Deref, From, Into)]
pub struct EntityMetadata(Vec<Option<EntityMetadataValue>>);

impl Encoder for EntityMetadata {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        let mut i = 0;
        for metadata in self.iter() {
            if let Some(metadata) = metadata {
                buf.write_u8(i)?;
                buf.write_var_i32(metadata.get_metadata_type_id())?;
                metadata.encode_to_buffer(buf)?;
            }
            i += 1;
        }
        buf.write_u8(0xff)?;
        Ok(())
    }
}

impl EntityMetadata {
    pub fn with_capacity<const LENGTH: usize>() -> EntityMetadata {
        EntityMetadata(Vec::from(const { [EntityMetadataValue::NONE; LENGTH] }))
    }
}

#[derive(Deref, From, Into)]
pub struct Entity(pub EntityMetadata);

impl Entity {
    pub fn new() -> Entity {
        Entity(EntityMetadata::with_capacity::<7>())
    }

    pub fn get_entity_byte(&mut self) -> EntityMetadataValue {
        let b = EntityMetadataValue::Byte(EntityByte::Default.bits());
        if let Some(some) = unsafe { self.0 .0.get_unchecked_mut(0) } {
            some.clone()
        } else {
            let value = EntityMetadataValue::Byte(EntityByte::Default.bits());
            unsafe { self.0 .0[0] = Some(value.clone()) };
            value
        }
    }
}

pub struct LivingEntity(EntityMetadata);

impl LivingEntity {
    pub fn new() -> LivingEntity {
        LivingEntity(EntityMetadata::with_capacity::<14>())
    }
}

bitflags! {
    pub struct EntityByte: u8 {
        const IsOnFire = 0x00;
        const IsCrouching = 0x02;
        const IsSprinting = 0x08;
        const IsSwimming = 0x10;
        const IsInvisible = 0x20;
        const HasGlowingEffect = 0x40;
        const IsFlyingWithElytra = 0x80;
        const None = 0;
        const Default = 0;
    }
}
