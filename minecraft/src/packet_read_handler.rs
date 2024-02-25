use std::io::Result;

use bytes::BytesMut;
use common_server::selector::Socket;

use super::player::Player;

pub trait PacketReadHandler {
    fn handle_packet_read(socket: &mut Socket<Player>, value: BytesMut) -> Result<()>;
}
