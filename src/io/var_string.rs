use std::io::{Error, ErrorKind, Result};

use super::var_int::VarIntRead;

use super::var_int::VarIntWrite;

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
