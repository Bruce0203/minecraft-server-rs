use std::io::{Error, Read, Result, Write};

use crate::io::prelude::VarIntWrite;
use crate::io::prelude::{Decoder, Encoder, VarIntRead};

pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl Encoder for GameMode {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        buf.write_var_i32(match self {
            GameMode::Survival => 0,
            GameMode::Creative => 1,
            GameMode::Adventure => 2,
            GameMode::Spectator => 3,
        })?;
        Ok(())
    }
}

impl Decoder for GameMode {
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(match reader.read_var_i32()? {
            0 => GameMode::Survival,
            1 => GameMode::Creative,
            2 => GameMode::Adventure,
            3 => GameMode::Spectator,
            n => {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("unknown gamemode: {}", n),
                ))
            }
        })
    }
}
