use std::io::Result;

use crate::io::prelude::{
    BitSet, BitSetRead, BitSetWrite, Buffer, Decoder, DecoderDeref, Encoder, U8Read, VarIntRead, VarIntSizedVecRead, VarIntSizedVecWrite, VarIntWrite
};

pub struct Light {
    sky_mask: BitSet,
    block_mask: BitSet,
    empty_sky_mask: BitSet,
    empty_block_mask: BitSet,
    sky_lights: Vec<Vec<u8>>,
    block_lights: Vec<Vec<u8>>,
}

impl Light {
    pub fn new() -> Light {
        let bitset_size = (0 + 63) / 64;
        Light {
            sky_mask: BitSet::with_capacity(bitset_size),
            block_mask: BitSet::with_capacity(bitset_size),
            empty_sky_mask: BitSet::with_capacity(bitset_size),
            empty_block_mask: BitSet::with_capacity(bitset_size),
            sky_lights: Vec::from([const { vec![] }; 37]),
            block_lights: Vec::from([const { vec![] }; 37]),
        }
    }
}

impl Encoder for Light {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_bitset(&self.sky_mask)?;
        buf.write_bitset(&self.block_mask)?;
        buf.write_bitset(&self.empty_sky_mask)?;
        buf.write_bitset(&self.empty_block_mask)?;
        buf.write_var_i32(self.sky_lights.len() as i32)?;
        for bytes in &self.sky_lights {
            buf.write_var_int_sized_vec(bytes)?;
        }
        buf.write_var_i32(self.block_lights.len() as i32)?;
        for bytes in &self.block_lights {
            buf.write_var_int_sized_vec(bytes)?;
        }
        Ok(())
    }
}

impl Decoder for Light {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        impl !DecoderDeref for Vec<u8> {}
        impl Decoder for Vec<u8> {
            fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
                let len = reader.read_var_i32()?;
                let mut vec = Vec::with_capacity(len as usize);
                for value in 0..len {
                    vec.push(reader.read_u8()?);
                }
                Ok(vec)
            }
        }
        Ok(Light {
            sky_mask: reader.read_bitset()?,
            block_mask: reader.read_bitset()?,
            empty_sky_mask: reader.read_bitset()?,
            empty_block_mask: reader.read_bitset()?,
            sky_lights: reader.read_var_int_sized_vec()?,
            block_lights: reader.read_var_int_sized_vec()?,
        })
    }
}
