use std::{
    char::MAX,
    io::{Result, Seek, Write},
};

use serde::{ser::SerializeSeq, Deserialize, Serialize};

use crate::io::prelude::{
    BitSet, BitSetWrite, Buffer, Encoder, I16Write, NbtNetworkWrite, U16Write, U8Write,
    VarIntSizedVecWrite, VarIntWrite,
};

use super::{light::Light, palette::Palette};

pub struct Chunk {
    x: i32,
    z: i32,
    heightmaps: HeightMaps,
    sections: Vec<ChunkSection>,
    block_entities: Vec<BlockEntity>,
    light: Light,
}

impl Encoder for Chunk {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(self.x)?;
        buf.write_var_i32(self.z)?;
        buf.write_network_nbt(&self.heightmaps)?;
        let mut buf2 = Buffer::new(vec![]);
        self.sections.encode_to_buffer(&mut buf2)?;
        buf.write_var_i32(buf2.get_ref().len() as i32)?;
        buf.write_all(buf2.get_ref())?;
        buf.write_var_int_sized_vec(&self.block_entities)?;
        self.light.encode_to_buffer(buf)?;
        Ok(())
    }
}

impl Encoder for Vec<ChunkSection> {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        //length is not encoded
        for chunk_section in self.iter() {
            chunk_section.encode_to_buffer(buf)?;
        }
        Ok(())
    }
}

pub struct ChunkSection {
    block_states: Palette,
    biomes: Palette,
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

pub struct ChunkPos {
    x: i32,
    y: i32,
    z: i32,
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
