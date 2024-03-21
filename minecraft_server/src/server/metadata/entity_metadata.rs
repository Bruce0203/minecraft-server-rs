use std::io::{Result, Write};

use bitflags::bitflags;

use crate::{
    io::prelude::{
        Buffer, Encoder, OptionWrite, U8Write, VarIntWrite, VarString, VarStringWrite, WriteBool,
    },
    server::prelude::{Chat, Pose},
};

use super::prelude::{MetadataEncoder, MetadataField, MetadataType};
use derive_more::Deref;

#[derive(Default)]
pub struct EntityMeta {
    pub entity_byte: Option<EntityByte>,
    pub air_ticks: Option<i32>,
    pub custom_name: Option<Option<VarString<32767>>>,
    pub is_custom_name_visible: Option<i32>,
    pub is_silent: Option<bool>,
    pub has_no_gravity: Option<bool>,
    pub pose: Option<Pose>,
    pub ticks_frozen_in_powdered_snow: Option<i32>,
}

impl Encoder for EntityMeta {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        if let Some(entity_byte) = &self.entity_byte {
            buf.write_u8(0)?;
            buf.write_u8(0)?;
            buf.write_u8(entity_byte.bits())?;
        }
        if let Some(air_ticks) = self.air_ticks {
            buf.write_u8(1)?;
            buf.write_u8(1)?;
            buf.write_var_i32(air_ticks)?;
        }
        if let Some(custom_name) = &self.custom_name {
            buf.write_u8(2)?;
            buf.write_u8(5)?;
            buf.write_option(custom_name)?;
        }
        if let Some(is_custom_name_visible) = self.is_custom_name_visible {
            buf.write_u8(3)?;
            buf.write_u8(8)?;
            buf.write_var_i32(is_custom_name_visible)?;
        }
        if let Some(is_silent) = self.is_silent {
            buf.write_u8(4)?;
            buf.write_u8(8)?;
            buf.write_bool(is_silent)?;
        }
        if let Some(has_no_gravity) = self.has_no_gravity {
            buf.write_u8(5)?;
            buf.write_u8(8)?;
            buf.write_bool(has_no_gravity)?;
        }
        if let Some(pose) = self.pose {
            buf.write_u8(6)?;
            buf.write_u8(20)?;
            pose.encode_to_buffer(buf)?;
        }
        if let Some(ticks_frozen_in_powdered_snow) = self.ticks_frozen_in_powdered_snow {
            buf.write_u8(7)?;
            buf.write_u8(1)?;
            buf.write_var_i32(ticks_frozen_in_powdered_snow)?;
        }
        Ok(())
    }
}

impl EntityMeta {
    pub fn get_entity_byte(&self) -> &EntityByte {
        self.entity_byte
            .as_ref()
            .unwrap_or_else(|| &EntityByte::Default)
    }

    pub fn set_entity_byte(&mut self, entity_byte: EntityByte) {
        self.entity_byte = Some(entity_byte);
    }

    pub fn get_air_ticks(&self) -> i32 {
        self.air_ticks.unwrap_or_else(|| 300)
    }

    const EMPTY_STRING: &String = &String::new();
    pub fn get_custom_name<'a>(&'a self) -> &'a Option<VarString<32767>> {
        if let Some(value) = &self.custom_name {
            &value
        } else {
            &None
        }
    }
}

impl MetadataType for EntityByte {
    fn get_type_id() -> i32 {
        0
    }
}

impl MetadataField for EntityByte {
    fn get_index(&self) -> u8 {
        0
    }
}

impl Encoder for EntityByte {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_u8(self.bits())?;
        Ok(())
    }
}

bitflags! {
    #[derive(Deref)]
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
