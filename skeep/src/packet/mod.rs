use bytes::BytesMut;
use common_server::selector::Socket;
use std::io::Result;

use crate::player::Player;

mod handshake;

pub fn handle_packet(socket: &mut Socket<Player>, bytes: BytesMut) -> Result<()> {

    Ok(())
}
