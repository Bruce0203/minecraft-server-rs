use std::io::{Cursor, Error, Result};

use mc_io::primitives::U128Read;
use mc_io::var_string::VarStringRead;
use uuid::Uuid;

use crate::connection::packet_handler::PacketHandler;
use crate::connection::packet_writer::PacketWriter;
use crate::{
    connection::player::Player, protocol::v1_20_4::login_success::LoginSuccess, server::Server,
};

pub struct LoginStart {
    name: String,
    player_uuid: Uuid,
}

impl TryFrom<&mut Cursor<Vec<u8>>> for LoginStart {
    type Error = Error;

    fn try_from(value: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(LoginStart {
            name: value.read_var_string::<16>()?,
            player_uuid: Uuid::from_u128(value.read_u128()?),
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
