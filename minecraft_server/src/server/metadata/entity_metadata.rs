use std::{
    io::{Error, Result},
    ops::{Deref, Index, IndexMut},
};

use crate::{
    io::prelude::{Buffer, Encoder, Identifier, U8Write, VarInt, VarIntWrite},
    protocol::v1_20_4::play::set_entity_metadata::SetEntityMetadata,
};
use bitflags::bitflags;
use derive_more::{Deref, DerefMut, From, Into};
use dyn_clone::DynClone;
use uuid::Uuid;

use super::prelude::EntityMetadataValue;

#[derive(Debug, Deref, DerefMut, From, Into)]
pub struct EntityMetadata<const LEN: usize>([Option<EntityMetadataValue>; LEN]);

impl<const LEN: usize> Encoder for EntityMetadata<LEN> {
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

impl<const LEN: usize> EntityMetadata<LEN> {
    pub fn new() -> EntityMetadata<LEN> {
        EntityMetadata(const { [EntityMetadataValue::NONE; LEN] })
    }

    pub fn get_or_else<'a, F: FnOnce() -> (EntityMetadataValue)>(
        &'a mut self,
        index: usize,
        f: F,
    ) -> EntityMetadataValue {
        if let Some(value) = &self[index] {
            value.clone()
        } else {
            let value = f();
            self[index] = Some(value.clone());
            value
        }
    }
}

impl<const LEN: usize> Index<usize> for EntityMetadata<LEN> {
    type Output = Option<EntityMetadataValue>;

    fn index(&self, index: usize) -> &Self::Output {
        if let Some(value) = self.0.get(index) {
            value
        } else {
            &None
        }
    }
}

impl<const LEN: usize> IndexMut<usize> for EntityMetadata<LEN> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.as_mut().get_mut(index).expect("out of index")
    }
}

#[derive(Deref, DerefMut, From, Into)]
pub struct Entity(pub EntityMetadata<7>);

impl Entity {
    pub fn get_entity_byte(&mut self) -> u8 {
        let a = if let Some(some) = unsafe { self.0 .0.get_unchecked_mut(0) } {
            some.clone()
        } else {
            let value = EntityMetadataValue::Byte(EntityByte::Default.bits());
            unsafe { self.0 .0[0] = Some(value.clone()) };
            value
        };
        match a {
            EntityMetadataValue::Byte(value) => return value,
            _ => panic!(),
        }
    }
}

impl Default for Entity {
    fn default() -> Self {
        let mut meta = EntityMetadata::<7>::new();
        meta[0] = Some(EntityMetadataValue::Byte(0));
        Entity(meta)
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
        const Default = Self::None.bits();
    }
}

#[test]
fn test_metadata() {
    let mut meta = EntityMetadata::<2>::new();
    println!("{:#?}", meta[0]);
    //meta[0] = None;
    //println!("{:?}", meta[0]);
    meta[0] = Some(EntityMetadataValue::Byte(0));
}
