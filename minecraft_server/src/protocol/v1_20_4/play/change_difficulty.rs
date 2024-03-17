use std::io::Result;

use crate::io::prelude::{Buffer, Encoder, U8Write, WriteBool};

pub struct S2CChangeDifficulty {
    pub new_difficulty: Difficulty,
    pub difficulty_locked: bool
}

impl Encoder for S2CChangeDifficulty {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_u8(self.new_difficulty as u8)?;
        buf.write_bool(self.difficulty_locked)?;
        Ok(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Difficulty {
    Peaceful = 0,
    Easy = 1,
    Normal = 2,
    Hard = 3,
}
