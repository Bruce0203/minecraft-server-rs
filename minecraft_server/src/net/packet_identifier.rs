use std::io::Result;

use super::socket::Player;

pub trait PacketIdentifier: Sized {
    fn get_packet_id(&self, player: &mut Player) -> Result<i32>;
}
