use std::io::{prelude::Read, Result};

use crate::io::prelude::{
    Buffer, Decoder, Encoder, Identifier, IdentifierRead, IdentifierWrite, VarInt,
    VarIntSizedVecRead, VarIntSizedVecWrite,
};

pub struct UpdateTags {
    pub tags: Vec<Tags>,
}

pub struct Tags {
    pub registry: Identifier,
    pub tag: Tag,
}

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
        self.tag.encode_to_buffer(buf)?;
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
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(UpdateTags {
            tags: VarIntSizedVecRead::read_var_int_sized_vec(reader)?,
        })
    }
}

impl Decoder for Tags {
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Tags {
            registry: IdentifierRead::read_identifier(reader)?,
            tag: Tag::decode_from_read(reader)?,
        })
    }
}

impl Decoder for Tag {
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Tag {
            name: IdentifierRead::read_identifier(reader)?,
            entries: reader.read_var_int_sized_vec()?,
        })
    }
}
