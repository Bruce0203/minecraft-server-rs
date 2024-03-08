use std::io::{Result, Read};

use crate::{server::prelude::Server, io::prelude::Decoder};

use super::prelude::{Socket, PacketIdentifier, PacketHandler};

pub trait PacketReader<Server, Player> {
    fn read_packet(server: &mut Server, player: &mut Socket<Player>) -> Result<()>;
}

impl<Packet: Decoder + PacketHandler<Player>, Player> PacketReader<Server, Player> for Packet {
    fn read_packet(server: &mut Server, player: &mut Socket<Player>) -> Result<()> {
        player.read_packet::<Packet>(server)?;
        Ok(())
    }
}
