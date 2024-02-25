use common_server::selector::Socket;

use crate::{packet_handler::PacketHandler, player::Player};

pub struct StatusRequest {}


pub struct StatusResponse {
}


impl PacketHandler for StatusRequest {
    fn handle_packet(&self, value: &mut Socket<Player>) {
        //value.connection.session_relay.protocol_id
    }
}
