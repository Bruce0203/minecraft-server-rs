use std::io::{Result, Write};

use crate::{
    io::{
        array::VarIntSizedVecWrite, encoding::Encoder, primitives::WriteBool,
        var_string::VarStringWrite,
    },
    net::prelude::{PacketIdentnifier, Player},
};
use uuid::Uuid;

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

impl PacketIdentnifier for LoginSuccess {
    fn get_packet_id(&self, _socket: &mut Player) -> Result<i32> {
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
