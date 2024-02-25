use bytes::BytesMut;
use common_server::selector::Socket;

use super::player::Player;

pub trait PacketHandler {
    fn handle_packet(&self, value: &mut Socket<Player>);
}

