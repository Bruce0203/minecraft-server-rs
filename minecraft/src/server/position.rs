use std::io::{
    prelude::{Read, Write},
    Result,
};

use common_server::{
    encoding::{Decoder, Encoder},
    primitives::I64Read,
};

#[derive(Debug, PartialEq, Eq)]
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
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let encoded =
            ((self.x & 0x3FFFFFF) << 38) | ((self.z & 0x3FFFFFF) << 12) | (self.y & 0xFFF);
        writer.write_all(&encoded.to_be_bytes())?;
        Ok(())
    }
}

impl Decoder for Position {
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self> {
        let long = reader.read_i64()?;
        Ok(Position {
            x: (long >> 38) as i64,
            y: (long << 52 >> 52) as i64,
            z: (long << 26 >> 38) as i64,
        })
    }
}

#[test]
fn position_encoding() {
    let pos = Position::new(1, 5, 3);
    let mut encoded = pos.encode().unwrap();
    let decoded = Position::decode_from_read(&mut encoded).unwrap();
    assert_eq!(decoded, pos);
}
