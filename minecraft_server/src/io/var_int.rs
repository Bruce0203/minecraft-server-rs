use derive_more::{Deref, From, Into};
use std::io::{
    prelude::{Read, Write},
    Error, ErrorKind, Result,
};

use super::prelude::{Buffer, Decoder, DecoderDeref, Encoder, EncoderDeref};

#[derive(Deref, From, Into, Clone, Copy)]
pub struct VarInt(pub i32);

impl !EncoderDeref for VarInt {}

impl Encoder for VarInt {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(self.0)?;
        Ok(())
    }
}

impl !DecoderDeref for VarInt {}
impl Decoder for VarInt {
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(reader.read_var_i32()?.into())
    }
}

pub fn read_var_i32_fast(buf: &[u8]) -> Result<(i32, usize)> {
    let mut val = 0;
    for i in 0..5 {
        let byte = buf[i];
        val |= (byte as i32 & 0b01111111) << (i * 7);
        if byte & 0b10000000 == 0 {
            return Ok((val, i + 1));
        }
    }
    Err(Error::new(ErrorKind::InvalidInput, "VarInt is too large"))
}

pub trait VarIntRead {
    fn read_var_i32(&mut self) -> Result<i32>;
}

pub trait VarIntWrite {
    fn write_var_i32(&mut self, value: i32) -> Result<usize>;
}

impl<R> VarIntRead for R
where
    R: std::io::Read,
{
    fn read_var_i32(&mut self) -> Result<i32> {
        let mut val = 0;
        let buf = &mut [0u8];
        for i in 0..5 {
            self.read_exact(buf)?;
            let byte = buf[0];
            val |= (byte as i32 & 0b01111111) << (i * 7);
            if byte & 0b10000000 == 0 {
                return Ok(val);
            }
        }
        Err(Error::new(ErrorKind::InvalidInput, "VarInt is too large"))
    }
}

impl<W> VarIntWrite for W
where
    W: std::io::Write,
{
    fn write_var_i32(&mut self, value: i32) -> Result<usize> {
        let x = value as u64;
        let stage1 = (x & 0x000000000000007f)
            | ((x & 0x0000000000003f80) << 1)
            | ((x & 0x00000000001fc000) << 2)
            | ((x & 0x000000000fe00000) << 3)
            | ((x & 0x00000000f0000000) << 4);

        let leading = stage1.leading_zeros();

        let unused_bytes = (leading - 1) >> 3;
        let bytes_needed = 8 - unused_bytes;

        // set all but the last MSBs
        let msbs = 0x8080808080808080;
        let msbmask = 0xffffffffffffffff >> (((8 - bytes_needed + 1) << 3) - 1);

        let merged = stage1 | (msbs & msbmask);
        let bytes = merged.to_le_bytes();

        Ok(self.write(unsafe { bytes.get_unchecked(..bytes_needed as usize) })?)
    }
}

#[test]
fn test_var_i32() {
    fn test(first: Vec<u8>, second: i32) {
        let mut vec = Vec::new();
        vec.write_var_i32(second).unwrap();
        assert_eq!(first, vec);
        let read = std::io::Cursor::new(vec).read_var_i32().unwrap();
        assert_eq!(read, second);
    }
    test(vec![0x00], 0);
    test(vec![0x01], 1);
    test(vec![0x02], 2);
    test(vec![0x7f], 127);
    test(vec![0x80, 0x01], 128);
    test(vec![0xff, 0x01], 255);
    test(vec![0xdd, 0xc7, 0x01], 25565);
    test(vec![0xff, 0xff, 0x7f], 2097151);
    test(vec![0xff, 0xff, 0xff, 0xff, 0x07], 2147483647);
    test(vec![0xff, 0xff, 0xff, 0xff, 0x0f], -1);
    test(vec![0x80, 0x80, 0x80, 0x80, 0x08], -2147483648);
}
