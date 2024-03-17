use std::io::Result;

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
    entity_byte: Option<EntityByte>,
    air_ticks: Option<i32>,
    custom_name: Option<VarString<32767>>,
    is_custom_name_visible: Option<i32>,
    is_silent: Option<bool>,
    has_no_gravity: Option<bool>,
    pose: Option<Pose>,
    ticks_frozen_in_powdered_snow: Option<i32>,
}

impl Encoder for EntityMeta {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        if let Some(entity_byte) = &self.entity_byte {
            buf.write_u8(0)?;
            buf.write_u8(entity_byte.bits())?;
        }
        if let Some(air_ticks) = self.air_ticks {
            buf.write_u8(1)?;
            buf.write_var_i32(air_ticks)?;
        }
        if let Some(custom_name) = &self.custom_name {
            buf.write_var_string(custom_name)?;
        }
        if let Some(is_custom_name_visible) = self.is_custom_name_visible {
            buf.write_var_i32(is_custom_name_visible)?;
        }
        if let Some(is_silent) = self.is_silent {
            buf.write_bool(is_silent)?;
        }
        if let Some(has_no_gravity) = self.has_no_gravity {
            buf.write_bool(has_no_gravity)?;
        }
        if let Some(pose) = self.pose {
            pose.encode_to_buffer(buf)?;
        }
        if let Some(ticks_frozen_in_powdered_snow) = self.ticks_frozen_in_powdered_snow {
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
    pub fn get_custom_name(&self) -> String {
        if let Some(value) = &self.custom_name {
            value.to_string()
        } else {
            String::default()
        }
    }
}

impl MetadataType for EntityByte {
    fn get_type_id(&self) -> i32 {
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
