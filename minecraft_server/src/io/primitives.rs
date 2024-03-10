use derive_more::{Deref, From, Into};
use std::io::{Read, Result, Write};

use super::prelude::{Decoder, Encoder};

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

#[derive(Deref, From, Into)]
pub struct U8(u8);

impl Encoder for U8 {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.0)?;
        Ok(())
    }
}

impl Decoder for U8 {
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(reader.read_u8()?.into())
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

#[derive(Deref, From, Into)]
pub struct I8(i8);

impl Encoder for I8 {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_i8(self.0)
    }
}

impl Decoder for I8 {
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(reader.read_i8()?.into())
    }
}

pub trait I8Read {
    fn read_i8(&mut self) -> Result<i8>;
}

impl<W: Write> U8Write for W {
    fn write_u8(&mut self, value: u8) -> Result<()> {
        self.write_all(&[value])
    }
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
        self.write_all(&[value as u8])
    }
}

#[derive(Deref, From, Into)]
pub struct U16(u16);

impl Encoder for U16 {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16(self.0)?;
        Ok(())
    }
}

impl Decoder for U16 {
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(reader.read_u16()?.into())
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

#[derive(Deref, From, Into)]
pub struct I16(i16);

impl Encoder for I16 {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_i16(self.0)
    }
}

impl Decoder for I16 {
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(reader.read_i16()?.into())
    }
}

pub trait I16Read {
    fn read_i16(&mut self) -> Result<i16>;
}

pub trait I16Write {
    fn write_i16(&mut self, value: i16) -> Result<()>;
}

impl<R: Read> I16Read for R {
    fn read_i16(&mut self) -> Result<i16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(i16::from_be_bytes(buf))
    }
}

impl<W: Write> I16Write for W {
    fn write_i16(&mut self, value: i16) -> Result<()> {
        self.write_all(&value.to_be_bytes())?;
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

#[derive(Deref, From, Into)]
pub struct U128(u128);

impl Encoder for U128 {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u128(self.0)
    }
}

impl Decoder for U128 {
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(reader.read_u128()?.into())
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

#[derive(Deref, From, Into)]
pub struct F32(f32);

impl Encoder for F32 {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_f32(self.0)
    }
}

impl Decoder for F32 {
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(reader.read_f32()?.into())
    }
}

pub trait F32Read {
    fn read_f32(&mut self) -> Result<f32>;
}

pub trait F32Write {
    fn write_f32(&mut self, value: f32) -> Result<()>;
}

impl<R: Read> F32Read for R {
    fn read_f32(&mut self) -> Result<f32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(f32::from_be_bytes(buf))
    }
}

impl<W: Write> F32Write for W {
    fn write_f32(&mut self, value: f32) -> Result<()> {
        self.write_all(&value.to_be_bytes())?;
        Ok(())
    }
}


#[derive(Deref, From, Into)]
pub struct F64(f64);

impl Encoder for F64 {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_f64(self.0)
    }
}

impl Decoder for F64 {
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(reader.read_f64()?.into())
    }
}

pub trait F64Read {
    fn read_f64(&mut self) -> Result<f64>;
}

pub trait F64Write {
    fn write_f64(&mut self, value: f64) -> Result<()>;
}

impl<R: Read> F64Read for R {
    fn read_f64(&mut self) -> Result<f64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(f64::from_be_bytes(buf))
    }
}

impl<W: Write> F64Write for W {
    fn write_f64(&mut self, value: f64) -> Result<()> {
        self.write_all(&value.to_be_bytes())?;
        Ok(())
    }
}
