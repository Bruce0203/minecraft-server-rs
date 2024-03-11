use std::{
    io::{Read, Result, Write},
    ops::Deref,
};

use serde::de::value;

use super::{
    encoding::{Decoder, Encoder},
    primitives::{BoolRead, WriteBool},
};

pub trait OptionRead {
    fn read_option<D: Decoder>(&mut self) -> Result<Option<D>>;
}

pub trait OptionWrite {
    fn write_option<E: Encoder>(&mut self, value: &Option<E>) -> Result<()>;
}

impl<R: Read> OptionRead for R {
    fn read_option<D: Decoder>(&mut self) -> Result<Option<D>> {
        Ok(if self.read_bool()? {
            Some(D::decode_from_read(self)?)
        } else {
            None
        })
    }
}

impl<W: Write> OptionWrite for W {
    fn write_option<E: Encoder>(&mut self, value: &Option<E>) -> Result<()> {
        if let Some(encoder) = value {
            self.write_bool(true)?;
            encoder.encode_to_write(self)?;
        } else {
            self.write_bool(false)?;
        }
        Ok(())
    }
}
