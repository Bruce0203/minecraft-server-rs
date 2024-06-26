use std::{
    fmt::Debug,
    io::{prelude::Read, Result},
};

use crate::io::prelude::{
    Buffer, Decoder, Encoder, Identifier, IdentifierRead, IdentifierWrite, VarInt,
    VarIntSizedVecRead, VarIntSizedVecWrite,
};

pub struct UpdateTags {
    pub tags: Vec<Tags>,
}

impl Debug for UpdateTags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UpdateTags").finish()
    }
}

#[derive(Debug)]
pub struct Tags {
    pub registry: Identifier,
    pub tag: Vec<Tag>,
}

#[derive(Debug)]
pub struct Tag {
    pub name: Identifier,
    pub entries: Vec<VarInt>,
}

impl Encoder for UpdateTags {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_int_sized_vec(&self.tags)?;
        Ok(())
    }
}

impl Encoder for Tags {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        self.registry.encode_to_buffer(buf)?;
        buf.write_var_int_sized_vec(&self.tag)?;
        Ok(())
    }
}

impl Encoder for Tag {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        self.name.encode_to_buffer(buf)?;
        buf.write_var_int_sized_vec(&self.entries)?;
        Ok(())
    }
}

impl Decoder for UpdateTags {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(UpdateTags {
            tags: VarIntSizedVecRead::read_var_int_sized_vec(reader)?,
        })
    }
}

impl Decoder for Tags {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(Tags {
            registry: IdentifierRead::read_identifier(reader)?,
            tag: reader.read_var_int_sized_vec()?,
        })
    }
}

impl Decoder for Tag {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(Tag {
            name: IdentifierRead::read_identifier(reader)?,
            entries: reader.read_var_int_sized_vec()?,
        })
    }
}
