use std::io::Result;

use crate::{server::prelude::Server, io::prelude::Decoder};

use super::prelude::Socket;

pub trait PacketHandler<Player> {
    fn handle_packet(&self, server: &mut Server, player: &mut Socket<Player>) -> Result<()>;
}

