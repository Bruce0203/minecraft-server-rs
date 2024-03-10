use std::io::Cursor;
use std::io::Read;
use std::io::Result;
use std::io::Write;
use std::ops::Deref;

use super::prelude::Cache;
use super::prelude::Identifier;

pub trait Encoder: Sized {
    #[inline]
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()>;

    #[inline]
    fn encode(&self) -> Result<Cursor<Vec<u8>>> {
        let mut bytes = Cursor::new(Vec::new());
        self.encode_to_write(&mut bytes)?;
        Ok(bytes)
    }
}

pub trait Decoder: Sized {
    #[inline]
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self>;
}

