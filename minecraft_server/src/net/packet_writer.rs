use std::io::Result;

use crate::io::prelude::Encoder;

use super::prelude::{PacketIdentifier, Socket};

pub trait PacketWriter<Player>: PacketIdentifier<Player> + Encoder {
    fn send_packet(&self, player: &mut Socket<Player>) -> Result<()> {
        Socket::send_packet(player, self)?;
        Ok(())
    }
}

impl<P: Encoder + PacketIdentifier<T>, T> PacketWriter<T> for P {}
