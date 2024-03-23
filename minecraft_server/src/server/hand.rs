use std::io::{Error, Read, Result};

use crate::io::prelude::{Buffer, Decoder, Encoder, VarIntRead, VarIntWrite};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum MainHand {
    Left = 0,
    Right = 1,
}

impl Decoder for MainHand {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(match reader.read_var_i32()? {
            0 => MainHand::Left,
            1 => MainHand::Right,
            n => {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("not valid main hand: {}", n),
                ))
            }
        })
    }
}

impl Encoder for MainHand {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(match self {
            MainHand::Left => 0,
            MainHand::Right => 1,
        })?;
        Ok(())
    }
}
