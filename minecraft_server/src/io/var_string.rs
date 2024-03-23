use derive_more::{Deref, From, Into};
use std::io::prelude::Write;
use std::io::{Error, ErrorKind, Result};

use super::buffer::Buffer;
use super::prelude::Decoder;
use super::prelude::DecoderDeref;
use super::prelude::Encoder;
use super::prelude::EncoderDeref;
use super::var_int::VarIntRead;

use super::var_int::VarIntWrite;

#[derive(Debug, Deref, From, Into, Clone)]
pub struct VarString<const MAX_LENGTH: usize>(pub String);

impl<const MAX_LENGTH: usize> !EncoderDeref for VarString<MAX_LENGTH> {}
impl<const MAX_LENGTH: usize> !DecoderDeref for VarString<MAX_LENGTH> {}

impl<const MAX_LENGTH: usize> Encoder for VarString<MAX_LENGTH> {
    fn encode_to_buffer(&self, buf: &mut super::prelude::Buffer) -> Result<()> {
        buf.write_var_string(self)?;
        Ok(())
    }
}

impl<const MAX_LENGTH: usize> Decoder for VarString<MAX_LENGTH> {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(reader.read_var_string::<MAX_LENGTH>()?.into())
    }
}
pub trait VarStringRead {
    fn read_var_string<const MAX_LENGTH: usize>(&mut self) -> Result<String>;
}

pub trait VarStringWrite {
    fn write_var_string(&mut self, value: &str) -> Result<usize>;
}

impl<R> VarStringRead for R
where
    R: std::io::Read,
{
    fn read_var_string<const MAX_LENGTH: usize>(&mut self) -> Result<String> {
        let length = self.read_var_i32()? as usize;
        if length > MAX_LENGTH * 3 {
            return Result::Err(Error::new(
                ErrorKind::InvalidInput,
                "String is longer than maximum allowed length",
            ));
        }
        let mut buf = vec![0u8; length];
        self.read_exact(&mut buf)?;
        match String::from_utf8(buf) {
            Ok(string) => Ok(string.to_string()),
            Err(err) => Result::Err(Error::new(ErrorKind::InvalidInput, err)),
        }
    }
}

impl<W> VarStringWrite for W
where
    W: std::io::Write,
{
    fn write_var_string(&mut self, value: &str) -> Result<usize> {
        self.write_var_i32(value.len() as i32)?;
        Ok(self.write(value.as_bytes())?)
    }
}

#[test]
fn test_read_var_string() {
    let input = "ABCD";
    let mut vec = Vec::<u8>::new();
    let mut cursor = std::io::Cursor::new(&mut vec);
    cursor.write_var_string(input).unwrap();
    cursor.set_position(0);
    assert_eq!(cursor.read_var_string::<255>().unwrap(), input);
}
