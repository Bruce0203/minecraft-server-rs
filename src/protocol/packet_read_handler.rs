use std::io::{Cursor, Result};

use crate::server::prelude::{Player, Server};

pub trait PacketReadHandler {
    fn handle_packet_read(server: &mut Server, player: &mut Player) -> Result<()>;
}
