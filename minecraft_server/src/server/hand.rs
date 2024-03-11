use std::io::{Error, Read};

use crate::io::prelude::{Decoder, VarIntRead};

#[derive(Debug, Clone, Copy)]
pub enum MainHand {
    Left,
    Right,
}

impl Decoder for MainHand {
    fn decode_from_read<R: Read>(reader: &mut R) -> std::io::Result<Self> {
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
