use std::io::Result;

use crate::io::prelude::Encoder;

use super::prelude::{Player, PacketIdentifier};

pub trait PacketWriter: PacketIdentifier + Encoder {
    fn send_packet(&self, player: &mut Player) -> Result<()> {
        player.send_packet(self)?;
        Ok(())
    }
}

impl<P: Encoder + PacketIdentifier> PacketWriter for P {}
