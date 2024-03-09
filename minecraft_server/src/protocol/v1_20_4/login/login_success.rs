use std::io::{Result, Write};

use uuid::Uuid;

use crate::{
    io::prelude::{Encoder, VarIntSizedVecWrite, VarStringWrite as _, WriteBool},
    net::prelude::{PacketIdentifier, Socket},
    server::prelude::{LoginPlayer, LoginServer},
};

pub struct LoginSuccess {
    pub uuid: Uuid,
    pub username: String,
    pub properties: Vec<LoginProperty>,
}

pub struct LoginProperty {
    pub name: String,
    pub value: String,
    pub is_signed: bool,
    pub signature: Option<String>,
}

impl Encoder for LoginSuccess {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.uuid.into_bytes())?;
        writer.write_var_string(self.username.as_str())?;
        writer.write_var_int_sized_vec(&self.properties)?;
        Ok(())
    }
}

impl PacketIdentifier<LoginPlayer> for LoginSuccess {
    fn get_protocol_id(&self, player: &mut Socket<LoginPlayer>) -> Result<i32> {
        Ok(0x02)
    }
}

impl Encoder for LoginProperty {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_var_string(self.name.as_str())?;
        writer.write_var_string(self.value.as_str())?;
        writer.write_bool(self.is_signed)?;
        if let Some(signature) = &self.signature {
            writer.write_var_string(signature)?;
        }
        Ok(())
    }
}
