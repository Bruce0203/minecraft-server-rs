use std::io::Result;

use crate::{server::prelude::Server, io::prelude::Decoder};

use super::prelude::Player;

pub trait PacketHandler {
    fn handle_packet(&self, server: &mut Server, player: &mut Player) -> Result<()>;
}

