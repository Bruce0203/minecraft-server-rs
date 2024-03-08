use std::io::Result;

use super::socket::Socket;

pub trait PacketIdentifier<Player>: Sized {
    fn get_packet_id(&self, player: &mut Socket<Player>) -> Result<i32>;
}
