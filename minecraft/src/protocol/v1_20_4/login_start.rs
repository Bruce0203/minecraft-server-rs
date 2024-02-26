use std::io::Error;

use bytes::{Buf, BytesMut};
use common_server::{packet::PacketHandler, selector::Socket, var_string::VarStringRead};
use uuid::Uuid;

use crate::{player::Player, server::Server};

pub struct LoginStart {
    name: String,
    player_uuid: Uuid,
}

impl TryFrom<BytesMut> for LoginStart {
    type Error = Error;

    fn try_from(value: BytesMut) -> Result<Self, Self::Error> {
        let mut reader = value.reader();
        Ok(LoginStart {
            name: reader.read_var_string::<16>()?,
            player_uuid: Uuid::from_u128(reader.into_inner().get_u128()),
        })
    }
}

impl PacketHandler<Player, Server> for LoginStart {
    fn handle_packet(&self, server: &mut Server, value: &mut Socket<Player>) {
        println!(
            "LoginStart(name={:?}, player_uuid={:?})",
            self.name, self.player_uuid
        );
    } 
}
