use std::io::{Error, Result};

use bytes::{Buf, Bytes, BytesMut};
use common_server::{packet::PacketHandler, selector::Socket, var_string::VarStringRead};
use uuid::Uuid;

use crate::{
    connection::packet_writer::PacketWriter,
    player::Player,
    protocol::v1_20_4::{
        login_success::{self, LoginSuccess},
        set_compression::{self, SetCompression},
    },
    server::Server,
};

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
        let login_success = LoginSuccess {
            uuid: self.player_uuid,
            username: self.name.to_owned(),
            properties: Vec::new(),
        };
        //set_compression::set_compression(socket, 256)?;
        login_success.send_packet(socket)?;
        Ok(())
    }
}
