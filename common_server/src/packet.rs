use std::io::Result;

use crate::selector::Socket;

pub trait PacketHandler<Server, T: Socket> {
    fn handle_packet(&self, server: &mut Server, socket: &mut T) -> Result<()>;
}
