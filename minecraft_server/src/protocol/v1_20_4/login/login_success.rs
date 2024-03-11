use std::io::{Result, Write};

use uuid::Uuid;

use crate::{
    io::prelude::{Encoder, VarIntSizedVecWrite, VarStringWrite as _, WriteBool},
    net::prelude::{PacketId, Socket},
    server::prelude::{GamePlayer, GameServer},
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
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        buf.write_all(&self.uuid.into_bytes())?;
        buf.write_var_string(self.username.as_str())?;
        buf.write_var_int_sized_vec(&self.properties)?;
        Ok(())
    }
}

impl Encoder for LoginProperty {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        buf.write_var_string(self.name.as_str())?;
        buf.write_var_string(self.value.as_str())?;
        buf.write_bool(self.is_signed)?;
        if let Some(signature) = &self.signature {
            buf.write_var_string(signature)?;
        }
        Ok(())
    }
}
