use bitflags::bitflags;
use mc_io::{encoding::Decoder, var_int::VarIntRead, var_string::VarStringRead, primitives::{I8Read, BoolRead, U8Read}};
use std::io::{Cursor, Error, Read, Result};

use crate::{connection::{player::Player, packet_handler::PacketHandler}, server::Server};

#[derive(Debug)]
pub struct ClientInformation {
    pub locale: String,
    pub view_distance: i8,
    pub chat_mode: ChatMode,
    pub chat_colors: bool,
    pub display_skin_parts: DisplaySkinParts,
    pub main_hand: MainHand,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
}

impl TryFrom<&mut Cursor<Vec<u8>>> for ClientInformation {
    type Error = Error;

    fn try_from(value: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(ClientInformation {
            locale: value.read_var_string::<16>()?,
            view_distance: value.read_i8()?,
            chat_mode: ChatMode::decode_from_read(value)?,
            chat_colors: value.read_bool()?,
            display_skin_parts: DisplaySkinParts::from_bits_truncate(value.read_u8()?),
            main_hand: MainHand::decode_from_read(value)?,
            enable_text_filtering: value.read_bool()?,
            allow_server_listings: value.read_bool()?,
        })
    }
}

bitflags! {
    #[derive(Debug)]
    pub struct DisplaySkinParts : u8 {
        const Cape = 0b_0000_0001;
        const Jacket = 0b_0000_0010;
        const LeftSleeve = 0b_0000_0100;
        const RightSleeve = 0b_0000_1000;
        const LeftPantsLeg = 0b_0001_0000;
        const RightPantsLeg = 0b_0010_0000;
        const Hat = 0b_0100_0000;
    }
}

#[derive(Debug)]
pub enum ChatMode {
    Enabled,
    CommandOnly,
    Hidden,
}

impl Decoder for ChatMode {
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(match reader.read_var_i32()? {
            0 => Self::Enabled,
            1 => Self::CommandOnly,
            2 => Self::Hidden,
            n => {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("not valid chat mode: {}", n),
                ))
            }
        })
    }
}

#[derive(Debug)]
pub enum MainHand {
    Left,
    Right,
}

impl Decoder for MainHand {
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(match reader.read_var_i32()? {
            0 => MainHand::Left,
            1 => MainHand::Right,
            n => {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("not valid main hand: {}", n),
                ))
            }
        })
    }
}

impl PacketHandler<Server, Player> for ClientInformation {
    fn handle_packet(&self, server: &mut Server, player: &mut Player) -> Result<()> {
        Ok(())
    }
}
