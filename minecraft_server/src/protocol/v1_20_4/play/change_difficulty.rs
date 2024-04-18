use std::io::Result;

use crate::io::prelude::{BoolRead, Buffer, Decoder, Encoder, U8Read, U8Write, WriteBool};

#[derive(Debug)]
pub struct ChangeDifficultyS2c {
    pub new_difficulty: Difficulty,
    pub difficulty_locked: bool,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Difficulty {
    Peaceful = 0,
    Easy = 1,
    Normal = 2,
    Hard = 3,
}

impl Encoder for ChangeDifficultyS2c {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_u8(self.new_difficulty as u8)?;
        buf.write_bool(self.difficulty_locked)?;
        Ok(())
    }
}

impl Decoder for ChangeDifficultyS2c {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(ChangeDifficultyS2c {
            new_difficulty: unsafe { std::mem::transmute(reader.read_u8()?) },
            difficulty_locked: reader.read_bool()?,
        })
    }
}
