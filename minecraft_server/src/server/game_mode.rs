use std::io::{Error, Read, Result, Write};

use crate::io::prelude::{Buffer, U8Read, U8Write, VarIntWrite};
use crate::io::prelude::{Decoder, Encoder, VarIntRead};

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum GameMode {
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}

#[derive(Debug)]
pub struct GameModeOrUndefined(pub Option<GameMode>);

impl Encoder for GameModeOrUndefined {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        if self.0.is_none() {
            buf.write_u8(0)?;
        } else {
            self.encode_to_buffer(buf)?;
        }
        Ok(())
    }
}

impl Decoder for GameModeOrUndefined {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        let value = reader.read_u8()?;
        Ok(GameModeOrUndefined(if value == 0 {
            None
        } else {
            Some(unsafe { std::mem::transmute(value) })
        }))
    }
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
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
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
