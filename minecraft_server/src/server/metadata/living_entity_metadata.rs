use std::io::Result;

use bitflags::bitflags;

use crate::{
    io::prelude::{
        Buffer, Encoder, F32Write, OptionWrite, U8Write, VarInt, VarIntWrite, WriteBool,
    },
    server::prelude::Position,
};

use super::prelude::EntityMeta;

#[derive(Debug, Default)]
pub struct LivingEntityMeta {
    pub entity: EntityMeta,
    pub living_entity_byte: Option<LivingEntityByte>,
    pub health: Option<f32>,
    pub potion_effect_color: Option<VarInt>,
    pub is_potion_effect_ambient: Option<bool>,
    pub number_of_arrows_in_entity: Option<VarInt>,
    pub number_of_bee_stingers_in_entity: Option<VarInt>,
    pub location_of_the_bed_that_the_entity_is_currently_sleeping_in: Option<Option<Position>>,
}

bitflags! {
    #[derive(Debug)]
    pub struct LivingEntityByte : u8 {
        const IsHandActive = 0x01;
        const ActiveHand = 0x02;
        const IsIntRiptideSpinAttack = 0x04;
        const None = 0x00;
        const Default = Self::None.bits();
    }
}

impl Encoder for LivingEntityMeta {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        self.entity.encode_to_buffer(buf)?;
        if let Some(value) = &self.living_entity_byte {
            buf.write_u8(8)?;
            buf.write_u8(0)?;
            buf.write_u8(value.bits())?;
        }
        if let Some(value) = self.health {
            buf.write_u8(9)?;
            buf.write_u8(3)?;
            buf.write_f32(value)?;
        }
        if let Some(value) = self.potion_effect_color {
            buf.write_u8(10)?;
            buf.write_u8(1)?;
            buf.write_var_i32(*value)?;
        }
        if let Some(value) = self.is_potion_effect_ambient {
            buf.write_u8(11)?;
            buf.write_u8(8)?;
            buf.write_bool(value)?;
        }
        if let Some(value) = self.number_of_arrows_in_entity {
            buf.write_u8(12)?;
            buf.write_u8(1)?;
            buf.write_var_i32(*value)?;
        }
        if let Some(value) = self.number_of_bee_stingers_in_entity {
            buf.write_u8(13)?;
            buf.write_u8(1)?;
            buf.write_var_i32(*value)?;
        }
        if let Some(value) = &self.location_of_the_bed_that_the_entity_is_currently_sleeping_in {
            buf.write_u8(14)?;
            buf.write_u8(11)?;
            buf.write_option(value)?;
        }
        Ok(())
    }
}
