use std::io::{Read, Result, Write};

pub trait BoolRead {
    fn read_bool(&mut self) -> Result<bool>;
}

pub trait WriteBool {
    fn write_bool(&mut self, value: bool) -> Result<()>;
}

impl<R: Read> BoolRead for R {
    fn read_bool(&mut self) -> Result<bool> {
        let buf = &mut [0];
        self.read_exact(buf)?;
        Ok(unsafe { std::mem::transmute(buf[0]) })
    }
}

impl<W: Write> WriteBool for W {
    fn write_bool(&mut self, value: bool) -> Result<()> {
        self.write_all(&[value as u8])?;
        Ok(())
    }
}

pub trait I64Read {
    fn read_i64(&mut self) -> Result<i64>;
}

pub trait I64Write {
    fn write_i64(&mut self, value: i64) -> Result<()>;
}

impl<R: Read> I64Read for R {
    fn read_i64(&mut self) -> Result<i64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(i64::from_be_bytes(buf))
    }
}

impl<W: Write> I64Write for W {
    fn write_i64(&mut self, value: i64) -> Result<()> {
        self.write_all(&value.to_be_bytes())?;
        Ok(())
    }
}

pub trait U8Read {
    fn read_u8(&mut self) -> Result<u8>;
}

pub trait U8Write {
    fn write_u8(&mut self, value: u8) -> Result<()>;
}

impl<R: Read> U8Read for R {
    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }
}

impl<W: Write> U8Write for W {
    fn write_u8(&mut self, value: u8) -> Result<()> {
        self.write_all(&[value])?;
        Ok(())
    }
}

pub trait U16Read {
    fn read_u16(&mut self) -> Result<u16>;
}

impl<R: Read> U16Read for R {
    fn read_u16(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }
}

pub trait U16Write {
    fn write_u16(&mut self, value: u16) -> Result<()>;
}

impl<W: Write> U16Write for W {
    fn write_u16(&mut self, value: u16) -> Result<()> {
        self.write_all(&value.to_be_bytes())?;
        Ok(())
    }
}

pub trait U128Read {
    fn read_u128(&mut self) -> Result<u128>;
}

pub trait U128Write {
    fn write_u128(&mut self, value: u128) -> Result<()>;
}

impl<R: Read> U128Read for R {
    fn read_u128(&mut self) -> Result<u128> {
        let mut buf = [0; 16];
        self.read_exact(&mut buf)?;
        Ok(u128::from_be_bytes(buf))
    }
}

impl<W: Write> U128Write for W {
    fn write_u128(&mut self, value: u128) -> Result<()> {
        self.write_all(&value.to_be_bytes())?;
        Ok(())
    }
}

pub trait I8Read {
    fn read_i8(&mut self) -> Result<i8>;
}

impl<R: Read> I8Read for R {
    fn read_i8(&mut self) -> Result<i8> {
        let mut buf = [0];
        self.read_exact(&mut buf)?;
        Ok(buf[0] as i8)
    }
}

pub trait I8Write {
    fn write_i8(&mut self, value: i8) -> Result<()>;
}

impl<W: Write> I8Write for W {
    fn write_i8(&mut self, value: i8) -> Result<()> {
        self.write_all(&[value as u8])?;
        Ok(())
    }
}
