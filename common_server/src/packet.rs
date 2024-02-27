use std::io::{Result, Write};

use crate::selector::Socket;

pub trait PacketHandler<Server, Player> {
    fn handle_packet(&self, server: &mut Server, player: &mut Socket<Player>) -> Result<()>;
}
