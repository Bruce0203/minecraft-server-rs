use std::io::prelude::Write;
use std::io::{Cursor, Error, Result};

use crate::io::prelude::{
    Buffer, Decoder, Encoder, U128Read, UuidWrite, VarStringRead, VarStringWrite,
};
use crate::net::prelude::{PacketHandler, PacketId, PacketWriter, Socket};
use crate::protocol::v1_20_4::login::login_success::LoginSuccess;
use crate::protocol::v1_20_4::login::set_compression;
use crate::server::prelude::{GamePlayer, GameServer};
use uuid::Uuid;

#[derive(Debug)]
pub struct LoginStart {
    pub name: String,
    pub player_uuid: Uuid,
}

impl Decoder for LoginStart {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(LoginStart {
            name: reader.read_var_string::<16>()?,
            player_uuid: Uuid::from_u128(reader.read_u128()?),
        })
    }
}

impl PacketHandler<GameServer> for LoginStart {
    fn handle_packet(
        &self,
        server: &mut GameServer,
        socket: &mut Socket<GamePlayer>,
    ) -> Result<()> {
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

impl Encoder for LoginStart {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        buf.write_var_string(&self.name)?;
        buf.write_uuid(self.player_uuid)?;
        Ok(())
    }
}
