use std::io::{prelude::Write, Error, ErrorKind, Result};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::io::prelude::{Decoder, Encoder, VarIntRead, VarIntWrite};

#[derive(Clone, Copy)]
#[repr(i32)]
#[derive(FromPrimitive, Default)]
pub enum Pose {
    #[default]
    Standing = 0,
    FallFlying = 1,
    Sleeping = 2,
    Swimming = 3,
    SpinAttack = 4,
    Sneaking = 5,
    LongJumping = 6,
    Dying = 7,
    Croaking = 8,
    UsingTongue = 9,
    Sitting = 10,
    Roaring = 11,
    Sniffing = 12,
    Emerging = 13,
    Digging = 14,
}

impl Encoder for Pose {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_var_i32(*self as i32)?;
        Ok(())
    }
}

impl Decoder for Pose {
    fn decode_from_read<R: std::io::prelude::Read>(reader: &mut R) -> Result<Self> {
        match FromPrimitive::from_i32(reader.read_var_i32()?) {
            Some(value) => Ok(value),
            None => Err(Error::new(ErrorKind::InvalidData, "pose is invalid")),
        }
    }
}
