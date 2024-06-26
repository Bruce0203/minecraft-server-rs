use std::io::{
    prelude::{Read, Write},
    Result,
};

use crate::io::prelude::{
    Buffer, Decoder, Encoder, F32Read, F32Write, F64Read, F64Write, I64Read, U8Read, U8Write,
    VarIntWrite,
};

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
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
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


#[derive(Debug)]
pub struct Angle(pub u8);

impl Encoder for Angle {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_u8(self.0)?;
        Ok(())
    }
}

impl Decoder for Angle {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(Angle(reader.read_u8()?))
    }
}

#[derive(Debug, Clone, Copy)]
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

impl Decoder for FloatRotation {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(FloatRotation {
            yaw: reader.read_f32()?,
            pitch: reader.read_f32()?,
        })
    }
}

impl Decoder for DoublePosition {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(DoublePosition {
            x: reader.read_f64()?,
            y: reader.read_f64()?,
            z: reader.read_f64()?,
        })
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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

impl Decoder for Location {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(Location {
            pos: DoublePosition::decode_from_read(reader)?,
            rot: FloatRotation::decode_from_read(reader)?,
        })
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Clone, Copy)]
pub struct Vector3(f32, f32, f32);

impl Encoder for Vector3 {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_f32(self.0)?;
        buf.write_f32(self.1)?;
        buf.write_f32(self.2)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
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
