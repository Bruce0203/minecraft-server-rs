use std::io::{Error, Result};

use bytes::{Buf, BytesMut, Bytes};
use common_server::{packet::PacketHandler, selector::Socket, var_string::VarStringRead};
use uuid::Uuid;

use crate::{player::Player, server::Server, protocol::v1_20_4::{set_compression::{self, SetCompression}, login_success::{self, LoginSuccess}}, connection::packet_writer::PacketWriter};

pub struct LoginStart {
    name: String,
    player_uuid: Uuid,
}

impl TryFrom<&mut BytesMut> for LoginStart {
    type Error = Error;

    fn try_from(value: &mut BytesMut) -> Result<Self> {
        let mut reader = value.reader();
        Ok(LoginStart {
            name: reader.read_var_string::<16>()?,
            player_uuid: Uuid::from_u128(reader.into_inner().get_u128()),
        })
    }
}

impl PacketHandler<Server, Player> for LoginStart {
    fn handle_packet(&self, server: &mut Server, socket: &mut Player) -> Result<()> {
        println!(
            "LoginStart(name={:?}, player_uuid={:?})",
            self.name, self.player_uuid
        );
        set_compression::set_compression(socket, 256)?;
        let login_success = LoginSuccess {
            uuid: self.player_uuid,
            username: self.name.to_owned(), 
            properties: Vec::new()
        };
        login_success.send_packet(socket)?;
        Ok(())
    }
}
