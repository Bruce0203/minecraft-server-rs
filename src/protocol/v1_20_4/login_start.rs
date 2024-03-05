use std::io::{Cursor, Error, Result};

use crate::io::primitives::U128Read;
use crate::io::var_string::VarStringRead;
use uuid::Uuid;

use crate::protocol::prelude::{PacketHandler, PacketWriter};
use crate::protocol::v1_20_4::login_success::LoginSuccess;
use crate::protocol::v1_20_4::set_compression;
use crate::server::prelude::{Server, Player};

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

impl PacketHandler for LoginStart {
    fn handle_packet(&self, socket: &mut Player) -> Result<()> {
        println!(
            "LoginStart(name={:?}, player_uuid={:?})",
            self.name, self.player_uuid
        );
        let login_success = LoginSuccess {
            uuid: self.player_uuid,
            username: self.name.to_owned(),
            properties: Vec::new(),
        };
        //set_compression::set_compression(socket, 20)?;
        login_success.send_packet(socket)?;
        Ok(())
    }
}
