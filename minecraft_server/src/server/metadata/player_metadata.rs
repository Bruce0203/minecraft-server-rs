use std::io::Result;

use bitflags::bitflags;

use crate::{
    io::prelude::{Buffer, Encoder, F32Write, NbtNetworkWrite, U8Write, VarInt, VarIntWrite},
    server::prelude::MainHand,
};

use super::living_entity_metadata::LivingEntityMeta;

#[derive(Default, Debug)]
pub struct PlayerMeta {
    pub living_entity: LivingEntityMeta,
    pub additional_hearts: Option<f32>,
    pub score: Option<VarInt>,
    pub player_byte: Option<PlayerByte>,
    pub main_hand: Option<MainHand>,
    pub left_shoulder_entity_data: Option<nbt::Value>,
    pub right_shoulder_entity_data: Option<nbt::Value>,
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::From, derive_more::Into)]
    pub struct PlayerByte : u8 {
        const CapeEnabled = 0x01;
        const JacketEnabled = 0x02;
        const LeftSleeveEnabled = 0x04;
        const RightSleeveEnabled = 0x08;
        const LeftPantsLegEnabled = 0x10;
        const RightPantsLegEnabled = 0x40;
        const Unused = 0x80;
        const None = 0;
        const Default = Self::None.bits();
    }
}
impl PlayerByte {
    pub const AllEnabled: u8 = const {
        PlayerByte::CapeEnabled.bits()
            | PlayerByte::JacketEnabled.bits()
            | PlayerByte::LeftSleeveEnabled.bits()
            | PlayerByte::RightSleeveEnabled.bits()
            | PlayerByte::LeftPantsLegEnabled.bits()
            | PlayerByte::RightPantsLegEnabled.bits()
    };
}

impl Encoder for PlayerMeta {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        self.living_entity.encode_to_buffer(buf)?;
        if let Some(value) = self.additional_hearts {
            buf.write_u8(15)?;
            buf.write_u8(3)?;
            buf.write_f32(value)?;
        }
        if let Some(value) = self.score {
            buf.write_u8(16)?;
            buf.write_u8(1)?;
            buf.write_var_i32(*value)?;
        }
        if let Some(value) = self.player_byte {
            buf.write_u8(17)?;
            buf.write_u8(0)?;
            buf.write_u8(value.bits())?;
        }
        if let Some(value) = self.main_hand {
            buf.write_u8(18)?;
            buf.write_u8(0)?;
            buf.write_u8(value as u8)?;
        }
        if let Some(value) = &self.left_shoulder_entity_data {
            buf.write_u8(19)?;
            buf.write_u8(16)?;
            todo!()
        }
        if let Some(value) = &self.right_shoulder_entity_data {
            buf.write_u8(20)?;
            buf.write_u8(16)?;
            todo!()
        }
        Ok(())
    }
}
