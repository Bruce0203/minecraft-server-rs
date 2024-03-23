use crate::{
    io::prelude::{
        BoolRead, Buffer, Decoder, DecoderDeref, Encoder, EncoderDeref, I8Read, I8Write, U8Read,
        U8Write, VarIntRead, VarIntWrite, VarString, VarStringRead, WriteBool,
    },
    net::prelude::{PacketHandler, Socket},
    server::prelude::{GamePlayer, GameServer, MainHand},
};
use bitflags::bitflags;
use derive_more::{Deref, From};
use std::io::{Cursor, Error, Read, Result};

#[derive(Debug, Deref, From)]
pub struct ClientInformationConf(pub ClientInformation);
#[derive(Debug, Deref, From)]
pub struct ClientInformationPlay(pub ClientInformation);

impl !DecoderDeref for ClientInformationPlay {}
impl !EncoderDeref for ClientInformationPlay {}

impl Encoder for ClientInformationConf {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        self.0.encode_to_buffer(buf)
    }
}

impl Encoder for ClientInformationPlay {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        self.0.encode_to_buffer(buf)
    }
}

impl Decoder for ClientInformationPlay {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(ClientInformationPlay(ClientInformation::decode_from_read(
            reader,
        )?))
    }
}

impl Decoder for ClientInformationConf {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(ClientInformationConf(ClientInformation::decode_from_read(
            reader,
        )?))
    }
}

#[derive(Debug)]
pub struct ClientInformation {
    pub locale: VarString<16>,
    pub view_distance: i8,
    pub chat_mode: ChatMode,
    pub chat_colors: bool,
    pub display_skin_parts: DisplaySkinParts,
    pub main_hand: MainHand,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
}

impl Encoder for ClientInformation {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        self.locale.encode_to_buffer(buf)?;
        buf.write_i8(self.view_distance)?;
        self.chat_mode.encode_to_buffer(buf)?;
        buf.write_bool(self.chat_colors)?;
        self.display_skin_parts.encode_to_buffer(buf)?;
        self.main_hand.encode_to_buffer(buf)?;
        buf.write_bool(self.enable_text_filtering)?;
        buf.write_bool(self.allow_server_listings)?;
        Ok(())
    }
}

impl Decoder for ClientInformation {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(ClientInformation {
            locale: VarString::<16>::decode_from_read(reader)?,
            view_distance: reader.read_i8()?,
            chat_mode: ChatMode::decode_from_read(reader)?,
            chat_colors: reader.read_bool()?,
            display_skin_parts: DisplaySkinParts::from_bits_truncate(reader.read_u8()?),
            main_hand: MainHand::decode_from_read(reader)?,
            enable_text_filtering: reader.read_bool()?,
            allow_server_listings: reader.read_bool()?,
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
        const None = 0;
    }
}

impl Encoder for DisplaySkinParts {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_u8(self.bits())?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum ChatMode {
    Enabled,
    CommandOnly,
    Hidden,
}

impl Decoder for ChatMode {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
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

impl Encoder for ChatMode {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(match self {
            ChatMode::Enabled => 0,
            ChatMode::CommandOnly => 1,
            ChatMode::Hidden => 2,
        })?;
        Ok(())
    }
}

impl PacketHandler<GameServer> for ClientInformation {
    fn handle_packet(
        &self,
        server: &mut GameServer,
        _player: &mut Socket<GamePlayer>,
    ) -> Result<()> {
        println!("{:#?}", self);
        Ok(())
    }
}

impl PacketHandler<GameServer> for ClientInformationPlay {
    fn handle_packet(
        &self,
        server: &mut GameServer,
        player: &mut Socket<GamePlayer>,
    ) -> Result<()> {
        self.0.handle_packet(server, player)?;
        Ok(())
    }
}

impl PacketHandler<GameServer> for ClientInformationConf {
    fn handle_packet(
        &self,
        server: &mut GameServer,
        player: &mut Socket<GamePlayer>,
    ) -> Result<()> {
        self.0.handle_packet(server, player)?;
        Ok(())
    }
}
