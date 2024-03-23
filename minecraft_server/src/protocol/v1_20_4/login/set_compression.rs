use std::io::{prelude::Read, Result, Write};

use crate::{
    io::prelude::{Buffer, Decoder, Encoder, VarIntRead, VarIntWrite},
    net::prelude::{PacketHandler, PacketId, PacketWriter, Socket},
    server::prelude::{GamePlayer, GameServer},
};

#[derive(Debug)]
pub struct SetCompression {
    pub compression_threshold: i32,
}

impl Encoder for SetCompression {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(self.compression_threshold)?;
        Ok(())
    }
}

impl Decoder for SetCompression {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(SetCompression {
            compression_threshold: VarIntRead::read_var_i32(reader)?,
        })
    }
}

pub fn set_compression(socket: &mut Socket<GamePlayer>, compression_threshold: i32) -> Result<()> {
    let set_compression = SetCompression {
        compression_threshold,
    };
    set_compression.send_packet(socket)?;
    socket.session_relay.compression_threshold = compression_threshold;
    Ok(())
}
