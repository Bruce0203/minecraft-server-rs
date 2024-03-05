use std::io::Result;

use crate::io::encoding::Encoder;
use crate::server::prelude::Player;

pub trait PacketWriter: Sized + Encoder {
    fn get_packet_id(&self, player: &mut Player) -> Result<i32>;

    fn send_packet(&self, player: &mut Player) -> Result<()> {
        player.send_packet(self)
    }
}
