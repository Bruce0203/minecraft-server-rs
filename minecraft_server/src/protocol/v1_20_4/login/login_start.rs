use std::io::prelude::Write;
use std::io::{Cursor, Error, Result};

use crate::io::prelude::{Decoder, Encoder, U128Read, UuidWrite, VarStringRead, VarStringWrite};
use crate::net::prelude::{PacketHandler, PacketIdentifier, PacketWriter, Socket};
use crate::protocol::v1_20_4::login::login_success::LoginSuccess;
use crate::server::prelude::{LoginPlayer, LoginServer};
use uuid::Uuid;

pub struct LoginStart {
    pub name: String,
    pub player_uuid: Uuid,
}

impl Decoder for LoginStart {
    fn decode_from_read<R: std::io::prelude::Read>(reader: &mut R) -> Result<Self> {
        Ok(LoginStart {
            name: reader.read_var_string::<16>()?,
            player_uuid: Uuid::from_u128(reader.read_u128()?),
        })
    }
}

impl PacketHandler<LoginServer> for LoginStart {
    fn handle_packet(
        &self,
        server: &mut LoginServer,
        socket: &mut Socket<LoginPlayer>,
    ) -> Result<()> {
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

impl Encoder for LoginStart {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_var_string(&self.name)?;
        writer.write_uuid(self.player_uuid)?;
        Ok(())
    }
}
