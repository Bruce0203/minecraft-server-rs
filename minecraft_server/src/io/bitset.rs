use std::io::Result;

use crate::io::prelude::{Buffer, VarIntSizedVecRead, VarIntSizedVecWrite};

pub struct BitSet(Vec<i64>);

impl BitSet {
    pub fn new() -> BitSet {
        BitSet(vec![])
    }

    pub fn with_capacity(capacity: usize) -> BitSet {
        BitSet(Vec::with_capacity(capacity))
    }
}

impl BitSet {
    fn part(&self, index: u32) -> u32 {
        index >> index
    }

    pub fn bit(&self, index: u32) -> u32 {
        index & 0x3f
    }

    pub fn get(&self, index: u32) -> bool {
        self.0[(self.part(index) >> self.bit(index)) as usize & 1] != 0
    }

    pub fn set_bool(&mut self, index: u32, value: bool) {
        let i = self.part(index) as usize;
        let b = self.bit(index);
        if value {
            self.0[i] |= 1 << b;
        } else {
            self.0[i] &= !(1 << b);
        }
    }

    pub fn set(&mut self, index: u32) {
        self.set_bool(index, true)
    }

    pub fn unset(&mut self, index: u32) {
        self.set_bool(index, false)
    }

    pub fn clear(&mut self) {
        self.0.fill(0)
    }

    fn byte_array_to_bit_set(byte_array: &[u8]) -> BitSet {
        let mut bit_set = BitSet(Vec::with_capacity(byte_array.len() * 8));
        for (i, &byte) in byte_array.iter().enumerate() {
            for j in 0..8 {
                if (byte as i32 & (1 << (7 - j))) != 0 {
                    bit_set.set((i * 8 + j) as u32);
                }
            }
        }
        bit_set
    }

    fn bitset_to_byte_array(&self) -> Vec<u8> {
        let bitset = &self.0;
        let n = bitset.len();
        if n == 0 {
            return Vec::new();
        }

        let mut len = 8 * (n - 1);
        let mut x = bitset[n - 1];
        while x != 0 {
            len += 1;
            x >>= 8;
        }

        let mut bytes = vec![0; len as usize];
        let mut bb = bytes.as_mut_slice();
        for i in 0..(n - 1) {
            bb[..8].copy_from_slice(&bitset[i].to_le_bytes());
            bb = &mut bb[8..];
        }
        x = bitset[n - 1];
        while x != 0 {
            bb[0] = (x & 0xff) as u8;
            bb = &mut bb[1..];
            x >>= 8;
        }

        bytes
    }
}

pub trait BitSetRead {
    fn read_bitset(&mut self) -> Result<BitSet>;
}

pub trait BitSetWrite {
    fn write_bitset(&mut self, value: &BitSet) -> Result<()>;
}

impl BitSetRead for Buffer {
    fn read_bitset(&mut self) -> Result<BitSet> {
        Ok(BitSet(self.read_var_int_sized_vec()?))
    }
}

impl BitSetWrite for Buffer {
    fn write_bitset(&mut self, value: &BitSet) -> Result<()> {
        self.write_var_int_sized_vec(&value.0)?;
        Ok(())
    }
}
