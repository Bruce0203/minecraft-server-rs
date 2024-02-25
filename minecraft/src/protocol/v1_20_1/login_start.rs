use std::io::Error;

use bytes::{Buf, BytesMut};
use common_server::{selector::Socket, var_string::VarStringRead};
use uuid::Uuid;

use crate::{
    packet_handler::PacketHandler, player::Player,
};

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

impl PacketHandler for LoginStart {
    fn handle_packet(&self, system: &mut Socket<Player>) {
        println!(
            "LoginStart(name={:?}, player_uuid={:?}",
            self.name, self.player_uuid
        );
    }
}
