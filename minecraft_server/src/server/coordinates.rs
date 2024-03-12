use std::io::{
    prelude::{Read, Write},
    Result,
};

use crate::io::prelude::{Buffer, Decoder, Encoder, F32Write, F64Write, I64Read, VarIntWrite};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Position {
    pub fn new(x: i64, y: i64, z: i64) -> Position {
        Position { x, y, z }
    }
}

impl Encoder for Position {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        let encoded =
            ((self.x & 0x3FFFFFF) << 38) | ((self.z & 0x3FFFFFF) << 12) | (self.y & 0xFFF);
        buf.write_all(&encoded.to_be_bytes())?;
        Ok(())
    }
}

impl Decoder for Position {
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self> {
        let long = reader.read_i64()?;
        Ok(Position {
            x: (long >> 38),
            y: (long << 52 >> 52),
            z: (long << 26 >> 38),
        })
    }
}

#[test]
fn position_encoding() {
    let pos = Position::new(1, 5, 3);
    let mut encoded = pos.encode().unwrap();
    encoded.set_position(0);
    let decoded = Position::decode_from_read(&mut encoded).unwrap();
    assert_eq!(decoded, pos);
}

#[derive(derive_more::Deref)]
pub struct Angle(pub u8);

pub struct FloatRotation {
    pub yaw: f32,
    pub pitch: f32,
}

impl Encoder for FloatRotation {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_f32(self.yaw)?;
        buf.write_f32(self.pitch)?;
        Ok(())
    }
}

pub struct DoublePosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Encoder for DoublePosition {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_f64(self.x)?;
        buf.write_f64(self.y)?;
        buf.write_f64(self.z)?;
        Ok(())
    }
}

pub struct Location {
    pub pos: DoublePosition,
    pub rot: FloatRotation,
}

impl Encoder for Location {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        self.pos.encode_to_buffer(buf)?;
        self.rot.encode_to_buffer(buf)?;
        Ok(())
    }
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum Direction {
    Down = 0,
    Up = 1,
    North = 2,
    South = 3,
    West = 4,
    East = 5,
}

impl Encoder for Direction {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(*self as i32)?;
        Ok(())
    }
}

pub struct Quaternion(f32, f32, f32, f32);

impl Encoder for Quaternion {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        buf.write_f32(self.0)?;
        buf.write_f32(self.1)?;
        buf.write_f32(self.2)?;
        buf.write_f32(self.3)?;
        Ok(())
    }
}
