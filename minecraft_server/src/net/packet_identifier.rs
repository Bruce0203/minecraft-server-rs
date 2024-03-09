use std::io::Result;

use super::socket::Socket;

pub trait PacketIdentifier<Player>: Sized {
    #[inline]
    fn get_protocol_id(&self, player: &mut Socket<Player>) -> Result<i32>;
}
