use std::io::{Cursor, Result};

use crate::server::prelude::{Server, Player};

pub trait PacketReadHandler {
    fn handle_packet_read(
        server: &mut Server,
        socket: &mut Player,
        value: &mut Cursor<Vec<u8>>,
    ) -> Result<()>;
}
