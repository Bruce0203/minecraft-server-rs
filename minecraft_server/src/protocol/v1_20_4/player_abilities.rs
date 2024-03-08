use std::io::{
    prelude::{Read, Write},
    Result,
};

use bitflags::bitflags;

use crate::{
    io::prelude::{Decoder, Encoder, U8Write, F32Write, U8Read, F32Read},
    net::prelude::{PacketIdentifier, Player},
};

pub struct PlayerAbilities {
    pub flags: PlayerAbility,
    pub flying_speed: f32,
    pub field_of_view_modifier: f32,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct PlayerAbility: u8 {
        const Invulnerable = 0b_0000_0001;
        const Flying = 0b_0000_0010;
        const AllowFlying = 0b_0000_1000;
        const InstantBreak = 0b_1000_0000;
        const None = 0;
    }
}

impl Encoder for PlayerAbilities {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.flags.0 .0)?;
        writer.write_f32(self.flying_speed)?;
        writer.write_f32(self.field_of_view_modifier)?;
        Ok(())
    }
}

impl Decoder for PlayerAbilities {
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(PlayerAbilities {
            flags: PlayerAbility::from_bits_truncate(reader.read_u8()?),
            flying_speed: reader.read_f32()?,
            field_of_view_modifier: reader.read_f32()?,
        })
    }
}

impl PacketIdentifier for PlayerAbilities {
    fn get_packet_id(&self, player: &mut Player) -> Result<i32> {
        Ok(0x36)
    }
}
