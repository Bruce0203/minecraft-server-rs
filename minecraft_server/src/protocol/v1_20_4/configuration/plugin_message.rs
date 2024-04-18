use std::{
    fmt::Display,
    io::{BufRead, Cursor, Error, ErrorKind, Read, Result, Write},
};

use crate::{
    io::prelude::{
        Buffer, Decoder, DecoderDeref, Encoder, EncoderDeref, Identifier, ToIdentifier, VarString,
        VarStringRead,
    },
    net::prelude::{PacketHandler, Socket},
    server::prelude::{GamePlayer, GameServer},
};

use derive_more::{Deref, From, Into};

#[derive(Deref, From, Debug)]
pub struct PluginMessageConfC2s(pub PluginMessage);

impl !EncoderDeref for PluginMessageConfC2s {}
impl !DecoderDeref for PluginMessageConfC2s {}

impl Encoder for PluginMessageConfC2s {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        self.0.encode_to_buffer(buf)
    }
}

impl Decoder for PluginMessageConfC2s {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(PluginMessageConfC2s(PluginMessage::decode_from_read(
            reader,
        )?))
    }
}

#[derive(Debug, Deref, From)]
pub struct PluginMessagePlayC2s(pub PluginMessage);

impl !EncoderDeref for PluginMessagePlayC2s {}
impl !DecoderDeref for PluginMessagePlayC2s {}

impl Encoder for PluginMessagePlayC2s {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        self.0.encode_to_buffer(buf)
    }
}

impl Decoder for PluginMessagePlayC2s {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(PluginMessagePlayC2s(PluginMessage::decode_from_read(
            reader,
        )?))
    }
}

#[derive(Deref, From, Debug)]
pub struct PluginMessageConfS2c(pub PluginMessage);

impl !EncoderDeref for PluginMessageConfS2c {}
impl !DecoderDeref for PluginMessageConfS2c {}

impl Encoder for PluginMessageConfS2c {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        self.0.encode_to_buffer(buf)
    }
}

impl Decoder for PluginMessageConfS2c {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(PluginMessageConfS2c(PluginMessage::decode_from_read(
            reader,
        )?))
    }
}

#[derive(Deref, From, Debug)]
pub struct PluginMessagePlayS2c(pub PluginMessage);

impl Encoder for PluginMessagePlayS2c {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        self.0.encode_to_buffer(buf)
    }
}

impl Decoder for PluginMessagePlayS2c {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(PluginMessagePlayS2c(PluginMessage::decode_from_read(
            reader,
        )?))
    }
}

#[derive(Debug)]
pub struct PluginMessage {
    pub channel: String,
    pub data: VarString<32767>,
}

impl !EncoderDeref for PluginMessage {}
impl !DecoderDeref for PluginMessage {}

impl Encoder for PluginMessage {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        self.channel.to_identifier().encode_to_buffer(buf)?;
        self.data.encode_to_buffer(buf)?;
        Ok(())
    }
}

impl Decoder for PluginMessage {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        let channel = reader.read_var_string::<32767>()?;
        Ok(PluginMessage {
            channel,
            data: VarString::<32767>::decode_from_read(reader)?,
        })
    }
}

impl PacketHandler<GameServer> for PluginMessagePlayC2s {
    fn handle_packet(
        &self,
        server: &mut GameServer,
        player: &mut Socket<GamePlayer>,
    ) -> Result<()> {
        Ok(())
    }
}

impl PacketHandler<GameServer> for PluginMessageConfC2s {
    fn handle_packet(
        &self,
        server: &mut GameServer,
        player: &mut Socket<GamePlayer>,
    ) -> Result<()> {
        Ok(())
    }
}
