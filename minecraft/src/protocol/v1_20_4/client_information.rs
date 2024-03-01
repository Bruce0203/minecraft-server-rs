use std::{
    io::{Error, Read, Result, Write},
    ops::Deref,
};

use bitflags::{bitflags, Flags};
use bytes::{Buf, BytesMut};
use common_server::{
    encoding::{Decoder, Encoder},
    packet::PacketHandler,
    var_int::VarIntRead,
    var_string::VarStringRead,
};

use crate::{server::Server, connection::player::Player};

#[derive(Debug)]
pub struct ClientInformation {
    pub locale: String,
    pub view_distance: i8,
    pub chat_mode: ChatMode,
    chat_colors: bool,
    display_skin_parts: DisplaySkinParts,
    main_hand: MainHand,
    enable_text_filtering: bool,
    allow_server_listings: bool,
}

impl TryFrom<&mut BytesMut> for ClientInformation {
    type Error = Error;

    fn try_from(value: &mut BytesMut) -> Result<Self> {
        Ok(ClientInformation {
            locale: value.reader().read_var_string::<16>()?,
            view_distance: value.get_i8(),
            chat_mode: ChatMode::decode_from_read(&mut value.reader())?,
            chat_colors: unsafe { std::mem::transmute(value.get_u8()) },
            display_skin_parts: DisplaySkinParts::from_bits_truncate(value.get_u8()),
            main_hand: MainHand::decode_from_read(&mut value.reader())?,
            enable_text_filtering: unsafe { std::mem::transmute(value.get_u8()) },
            allow_server_listings: unsafe { std::mem::transmute(value.get_u8()) },
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
        println!("{:#?}", self);
        Ok(())
    }
}
