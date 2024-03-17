use std::io::Result;

use crate::io::prelude::{BitSet, BitSetWrite, Buffer, Encoder, VarIntSizedVecWrite};

pub struct Light {
    sky_mask: BitSet,
    block_mask: BitSet,
    empty_sky_mask: BitSet,
    empty_block_mask: BitSet,
    sky_lights: Vec<u8>,
    block_lights: Vec<u8>,
}

impl Encoder for Light {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_bitset(&self.sky_mask)?;
        buf.write_bitset(&self.block_mask)?;
        buf.write_bitset(&self.empty_sky_mask)?;
        buf.write_bitset(&self.empty_block_mask)?;
        buf.write_var_int_sized_vec(&self.sky_lights)?;
        buf.write_var_int_sized_vec(&self.block_lights)?;
        Ok(())
    }
}
