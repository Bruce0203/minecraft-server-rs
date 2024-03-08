use std::io::{Result, Read};

use crate::{server::prelude::Server, io::prelude::Decoder};

use super::prelude::{Player, PacketIdentifier, PacketHandler};

pub trait PacketReader {
    fn read_packet(server: &mut Server, player: &mut Player) -> Result<()>;
}

impl<Packet: Decoder + PacketHandler> PacketReader for Packet {
    fn read_packet(server: &mut Server, player: &mut Player) -> Result<()> {
        Packet::decode_from_read(&mut player.packet_buf)?.handle_packet(server, player)?;
        Ok(())
    }
}
