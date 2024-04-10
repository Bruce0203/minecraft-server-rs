use std::{collections::BTreeMap, io::Result, option::Option};

use super::metadata::prelude::BlockState;
use crate::io::prelude::{
    Buffer, Decoder, Encoder, U8Read, U8Write, VarInt, VarIntSizedVecRead, VarIntSizedVecWrite,
    VarIntWrite,
};

#[derive(Debug)]
pub struct Palette {
    pub bits_per_block: u8,
    pub palette: Vec<VarInt>,
    pub data: Vec<i64>,
}

impl Palette {
    pub const fn new(bits_per_block: u8) -> Palette {
        Palette {
            palette: vec![],
            data: vec![],
            bits_per_block,
        }
    }
}

impl Encoder for Palette {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_u8(self.bits_per_block)?;
        buf.write_var_int_sized_vec(&self.palette)?;
        buf.write_var_i32(0)?;
        Ok(())
    }
}

impl Decoder for Palette {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(Palette {
            bits_per_block: reader.read_u8()?,
            palette: reader.read_var_int_sized_vec()?,
            data: reader.read_var_int_sized_vec()?,
        })
    }
}

pub fn choose_palette(bits_per_block: u8) -> Palette {
    if bits_per_block <= 4 {
        Palette::new(4)
    } else {
        Palette::new(bits_per_block)
    }
}

fn get_global_palette_idfrom_state(state: &BlockState) -> u32 {
    (state.0 as u32)
}

fn get_state_from_global_palette_id<'a>(id: u32) -> BlockState {
    todo!()
}
