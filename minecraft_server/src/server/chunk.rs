use std::{char::MAX, io::Result};

use serde::{ser::SerializeSeq, Deserialize, Serialize};

use crate::io::prelude::{
    BitSet, BitSetWrite, Buffer, Encoder, I16Write, NbtNetworkWrite, U16Write, U8Write,
    VarIntSizedVecWrite, VarIntWrite,
};

pub struct Chunk {
    x: i32,
    z: i32,
    heightmaps: HeightMaps,
    chunk_data: ChunkData,
    block_entities: Vec<BlockEntity>,
    light: Light,
}

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

pub struct ChunkData(Vec<ChunkSection>);

impl Encoder for ChunkData {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        for chunk_section in self.0.iter() {
            chunk_section.encode_to_buffer(buf)?;
        }
        Ok(())
    }
}

pub struct ChunkSection {
    block_states: BlockPalette,
    biomes: BiomePalette,
}

impl Encoder for ChunkSection {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        self.block_states.encode_to_buffer(buf)?;
        self.biomes.encode_to_buffer(buf)?;
        Ok(())
    }
}

const fn data_array_size(default_bits_per_entry: usize, max_bits_per_entry: usize) -> usize {
    if default_bits_per_entry != 0 {
        let values_per_long = 64 / default_bits_per_entry;
        (max_bits_per_entry + values_per_long - 1) / values_per_long
    } else {
        max_bits_per_entry - 1
    }
}

pub struct Palette<
    const DIMESION: usize,
    const MAX_BITS_PER_ENTRY: usize,
    const DEFAULT_BITS_PER_ENTRY: usize,
> where
    [(); { data_array_size(DEFAULT_BITS_PER_ENTRY, MAX_BITS_PER_ENTRY) }]:,
{
    count: i32,
    palette: Vec<i32>,
    bits_per_array: usize,
    values: Vec<i64>,
}

pub struct ChunkPos {
    x: i32,
    y: i32,
    z: i32,
}

pub type BlockPalette = Palette<16, 8, 8>;
pub type BiomePalette = Palette<4, 3, 0>;

impl<
        const DIMESION: usize,
        const MAX_BITS_PER_ENTRY: usize,
        const DEFAULT_BITS_PER_ENTRY: usize,
    > Palette<DIMESION, MAX_BITS_PER_ENTRY, DEFAULT_BITS_PER_ENTRY>
where
    [(); { data_array_size(DEFAULT_BITS_PER_ENTRY, MAX_BITS_PER_ENTRY) }]:,
{
    fn get(&self, chunk_pos: ChunkPos) {
        let bits_per_array = self.bits_per_array;
    }

    pub fn has_palette(&self) -> bool {
        self.bits_per_array <= MAX_BITS_PER_ENTRY
    }

    pub fn get_palette_index(&mut self, value: i64) -> i64 {
        if self.has_palette() {
            return value;
        }
        let last_palette_index = self.values.len();
        let bpe = self.bits_per_array;
        if last_palette_index <= Self::max_palette_size(bpe) {
            self.resize(bpe + 1);
            return self.get_palette_index(value);
        }
        if let Some(value) = self.values.get(value as usize) {
            let value = *value as i64;
            value
        } else {
            let value = value as i64;
            self.values.push(value);
            value
        }
    }

    fn resize(&self, bpe: usize) {
        let new_bits_per_entry = if bpe > MAX_BITS_PER_ENTRY { 15 } else { bpe };
    }

    fn max_palette_size(bits_per_entry: usize) -> usize {
        1 << bits_per_entry
    }

    pub fn get_section_index(dimension: i32, pos: ChunkPos) -> i32 {
        let dimension_mask = dimension - 1;
        let dimension_bit_count = Self::bitsToRepresent(dimension_mask);
        (pos.y & dimension_mask) << (dimension_bit_count >> 1)
            | ((pos.z & dimension_mask) << dimension_bit_count)
            | (pos.x & dimension_mask)
    }

    pub fn bitsToRepresent(n: i32) -> u32 {
        32 - n.leading_zeros()
    }
}

impl<
        const DIMESION: usize,
        const MAX_BITS_PER_ENTRY: usize,
        const DEFAULT_BITS_PER_ENTRY: usize,
    > Encoder for Palette<DIMESION, MAX_BITS_PER_ENTRY, DEFAULT_BITS_PER_ENTRY>
where
    [(); { data_array_size(DEFAULT_BITS_PER_ENTRY, MAX_BITS_PER_ENTRY) }]:,
{
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        Ok(())
    }
}

#[derive(Serialize)]
pub struct HeightMaps {
    #[serde(rename = "MOTION_BLOCKING")]
    motion_blocking: LongArray<37>,
    world_surface: LongArray<37>,
}

#[derive(derive_more::Deref)]
pub struct LongArray<const LEN: usize>([i64; LEN]);

impl<const LEN: usize> Serialize for LongArray<LEN> {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_seq(Some(LEN))?;
        for ele in self.0.iter() {
            state.serialize_element(ele)?;
        }
        state.end()
    }
}

pub struct ByteArray<const LEN: usize>([u8; LEN]);

impl<const LEN: usize> Serialize for ByteArray<LEN> {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_seq(Some(LEN))?;
        for ele in self.0.iter() {
            state.serialize_element(ele)?;
        }
        state.end()
    }
}

pub struct BlockEntity {
    x: u8,
    y: i16,
    z: u8,
    entity_type: i32,
    data: BlockEntityNbt,
}

impl Encoder for BlockEntity {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_u8((self.x & 15) << 4 | (self.z & 15))?;
        buf.write_i16(self.y)?;
        buf.write_var_i32(self.entity_type)?;
        buf.write_network_nbt(&self.data)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct BlockEntityNbt {}
