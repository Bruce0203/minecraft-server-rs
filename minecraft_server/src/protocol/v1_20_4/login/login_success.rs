use std::io::{Read, Result, Write};

use uuid::Uuid;

use crate::{
    io::prelude::{
        BoolRead, Buffer, Decoder, Encoder, OptionRead, VarIntSizedVecRead, VarIntSizedVecWrite, VarString, VarStringRead, VarStringWrite as _, WriteBool
    },
    net::prelude::{PacketId, Socket},
    server::prelude::{GamePlayer, GameServer},
};

#[derive(Debug)]
pub struct LoginSuccess {
    pub uuid: Uuid,
    pub username: String,
    pub properties: Vec<LoginProperty>,
}

#[derive(Debug)]
pub struct LoginProperty {
    pub name: VarString<32767>,
    pub value: VarString<32767>,
    pub is_signed: bool,
    pub signature: Option<VarString<32767>>,
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

impl Decoder for LoginProperty {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(LoginProperty {
            name: VarString::decode_from_read(reader)?,
            value: VarString::decode_from_read(reader)?,
            is_signed: BoolRead::read_bool(reader)?,
            signature: OptionRead::read_option(reader)?,
        })
    }
}

impl Decoder for LoginSuccess {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(LoginSuccess {
            uuid: Uuid::decode_from_read(reader)?,
            username: VarStringRead::read_var_string::<16>(reader)?,
            properties: VarIntSizedVecRead::read_var_int_sized_vec(reader)?,
        })
    }
}
