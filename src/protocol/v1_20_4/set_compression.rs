use std::io::{Result, Write};

use mc_io::encoding::Encoder;
use mc_io::var_int::VarIntWrite;

use crate::connection::prelude::PacketWriter;
use crate::server::prelude::Player;

pub struct SetCompression {
    pub compression_threshold: i32,
}

impl Encoder for SetCompression {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_var_i32(self.compression_threshold)?;
        Ok(())
    }
}

impl PacketWriter for SetCompression {
    fn get_packet_id(&self, _socket: &mut Player) -> Result<i32> {
        Ok(0x03)
    }
}

pub fn set_compression(socket: &mut Player, compression_threshold: i32) -> Result<()> {
    let set_compression = SetCompression {
        compression_threshold,
    };
    set_compression.send_packet(socket)?;
    socket.session_relay.compression_threshold = compression_threshold;
    Ok(())
}