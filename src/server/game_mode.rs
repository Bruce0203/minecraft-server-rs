use std::io::{
    prelude::{Read, Write},
    Error, Result,
};

use crate::io::{encoding::{Encoder, Decoder}, var_int::{VarIntRead, VarIntWrite}};

pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl Encoder for GameMode {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_var_i32(match self {
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

