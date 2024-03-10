use std::io::{Result, Write};

use crate::{
    io::prelude::{Encoder, VarIntWrite},
    net::prelude::{PacketId, PacketWriter, Socket},
    server::prelude::{LoginPlayer, LoginServer},
};

pub struct SetCompression {
    pub compression_threshold: i32,
}

impl Encoder for SetCompression {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_var_i32(self.compression_threshold)?;
        Ok(())
    }
}


pub fn set_compression(socket: &mut Socket<LoginPlayer>, compression_threshold: i32) -> Result<()> {
    let set_compression = SetCompression {
        compression_threshold,
    };
    set_compression.send_packet(socket)?;
    socket.session_relay.compression_threshold = compression_threshold;
    Ok(())
}
