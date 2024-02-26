use std::io::Result;

use bytes::BytesMut;
use common_server::selector::Socket;

use crate::server::Server;

use super::player::Player;

pub trait PacketReadHandler {
    fn handle_packet_read(server: &mut Server, socket: &mut Socket<Player>, value: BytesMut) -> Result<()>;
}
