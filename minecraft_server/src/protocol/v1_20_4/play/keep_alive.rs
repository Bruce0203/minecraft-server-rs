use std::io::{prelude::Read, Result};

use crate::io::prelude::I64Read;
use crate::server::prelude::GamePlayer;
use crate::{
    io::prelude::{Buffer, Decoder, DecoderDeref, Encoder, EncoderDeref, I64Write},
    net::prelude::{PacketHandler, Server, Socket},
    server::prelude::GameServer,
};
use derive_more::{Deref, From, Into};

#[derive(Debug)]
pub struct KeepAlivePlayS2c(pub KeepAlive);

impl !EncoderDeref for KeepAlivePlayS2c {}
impl !DecoderDeref for KeepAlivePlayS2c {}

#[derive(Debug)]
pub struct KeepAlivePlayC2s(pub KeepAlive);

impl !EncoderDeref for KeepAlivePlayC2s {}
impl !DecoderDeref for KeepAlivePlayC2s {}

#[derive(Debug)]
pub struct KeepAliveConfC2s(pub KeepAlive);

impl !EncoderDeref for KeepAliveConfC2s {}
impl !DecoderDeref for KeepAliveConfC2s {}

#[derive(Debug)]
pub struct KeepAliveConfS2c(pub KeepAlive);

impl !EncoderDeref for KeepAliveConfS2c {}
impl !DecoderDeref for KeepAliveConfS2c {}

impl Encoder for KeepAlivePlayS2c {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        self.0.encode_to_buffer(buf)
    }
}

impl Encoder for KeepAlivePlayC2s {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        self.0.encode_to_buffer(buf)
    }
}

impl Decoder for KeepAlivePlayC2s {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(KeepAlivePlayC2s(KeepAlive(reader.read_i64()?)))
    }
}

impl Decoder for KeepAliveConfC2s {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(KeepAliveConfC2s(KeepAlive::decode_from_read(reader)?))
    }
}

impl Encoder for KeepAliveConfC2s {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        Ok(self.0.encode_to_buffer(buf)?)
    }
}

impl Encoder for KeepAliveConfS2c {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        self.0.encode_to_buffer(buf)
    }
}

impl Decoder for KeepAliveConfS2c {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(KeepAliveConfS2c(KeepAlive::decode_from_read(reader)?))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KeepAlive(pub i64);

impl Encoder for KeepAlive {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_i64(self.0)?;
        Ok(())
    }
}

impl Decoder for KeepAlive {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(KeepAlive(reader.read_i64()?))
    }
}

impl PacketHandler<GameServer> for KeepAlivePlayC2s {
    fn handle_packet(
        &self,
        server: &mut GameServer,
        player: &mut Socket<GamePlayer>,
    ) -> Result<()> {
        println!("keepalive!");
        Ok(())
    }
}
