use common_server::selector::Socket;

use crate::player::Player;

pub trait PacketHandler {
    fn handle_packet(&self, value: &mut Socket<Player>);
}
