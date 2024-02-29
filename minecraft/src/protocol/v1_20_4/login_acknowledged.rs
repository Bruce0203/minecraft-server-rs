use std::io::{Error, Result};

use bytes::BytesMut;
use common_server::packet::PacketHandler;

use crate::{prelude::Player, server::Server, connection::ConnectionState};

pub struct LoginAcknowledged {}

impl TryFrom<&mut BytesMut> for LoginAcknowledged {
    type Error = Error;

    fn try_from(value: &mut BytesMut) -> Result<Self> {
        Ok(LoginAcknowledged {})
    }
}

impl PacketHandler<Server, Player> for LoginAcknowledged {
    fn handle_packet(&self, server: &mut Server, player: &mut Player) -> Result<()> {
        player.session_relay.connection_state = ConnectionState::Confgiuration;
        Ok(())
    }
}
