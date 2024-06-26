use std::io::{
    prelude::{Read, Write},
    Result,
};

use super::{
    buffer::Buffer,
    encoding::{Decoder, Encoder},
    prelude::{DecoderDeref, EncoderDeref},
    var_string::{VarStringRead, VarStringWrite},
};

#[derive(Debug, derive_more::Deref, Clone)]
pub struct Identifier(String);

pub trait ToIdentifier {
    fn to_identifier(&self) -> Identifier;
}

impl<S: ToString> ToIdentifier for S {
    fn to_identifier(&self) -> Identifier {
        Identifier(self.to_string())
    }
}

impl !DecoderDeref for Identifier {}

impl !EncoderDeref for Identifier {}

impl Decoder for Identifier {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(Identifier(reader.read_var_string::<32767>()?))
    }
}

impl Encoder for Identifier {
    fn encode_to_buffer(&self, buf: &mut super::prelude::Buffer) -> Result<()> {
        buf.write_var_string(&self.0)?;
        Ok(())
    }
}

pub trait IdentifierRead {
    fn read_identifier(&mut self) -> Result<Identifier>;
}

pub trait IdentifierWrite {
    fn write_identifier(&mut self, value: Identifier) -> Result<()>;
}

impl<R: Read> IdentifierRead for R {
    fn read_identifier(&mut self) -> Result<Identifier> {
        let value = Ok(Identifier(self.read_var_string::<32767>()?));
        value
    }
}
