use std::{
    char::MAX,
    io::{Result, Seek, Write},
};

use serde::{ser::SerializeSeq, Deserialize, Serialize};

use crate::io::prelude::{
    BitSet, BitSetWrite, Buffer, Encoder, I16Write, I32Write, NbtNetworkWrite, U16Write, U8Write,
    VarIntSizedVecWrite, VarIntWrite,
};

use super::{light::Light, palette::Palette};

pub struct Chunk {
    pub x: i32,
    pub z: i32,
    pub heightmaps: HeightMaps,
    pub sections: Vec<ChunkSection>,
    pub block_entities: Vec<BlockEntity>,
    pub light: Light,
}

impl Chunk {
    pub fn new(x: i32, z: i32) -> Chunk {
        Chunk {
            x,
            z,
            heightmaps: HeightMaps::new(),
            sections: Vec::from([const { ChunkSection::new() }; 24]),
            block_entities: vec![],
            light: Light::new(),
        }
    }
}

impl Encoder for Chunk {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_i32(self.x)?;
        buf.write_i32(self.z)?;
        self.heightmaps.encode_to_buffer(buf)?;
        let mut buf2 = Buffer::new(vec![]);
        self.sections.encode_to_buffer(&mut buf2)?;
        buf.write_var_i32(buf2.get_ref().len() as i32)?;
        buf.write_all(buf2.get_ref())?;
        buf.write_var_int_sized_vec(&self.block_entities)?;
        self.light.encode_to_buffer(buf)?;
        Ok(())
    }
}

#[test]
fn test_chunk() {
    let chunk = Chunk::new(0, 0);
    println!("{:?}", *chunk.sections.encode().unwrap().get_ref());
    println!("{:?}", chunk.sections.encode().unwrap().get_ref().len());
}

pub struct ChunkSection {
    block_states: Palette,
    biomes: Palette,
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

impl ChunkSection {
    pub const fn new() -> ChunkSection {
        ChunkSection {
            block_states: Palette::new(0),
            biomes: Palette::new(0),
        }
    }
}

impl Encoder for ChunkSection {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_i16(0)?;
        self.block_states.encode_to_buffer(buf)?;
        self.biomes.encode_to_buffer(buf)?;
        Ok(())
    }
}

pub struct ChunkPos {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeightMaps {
    #[serde(serialize_with = "nbt::i64_array")]
    #[serde(rename = "MOTION_BLOCKING")]
    pub motion_blocking: Vec<i64>,
    #[serde(serialize_with = "nbt::i64_array")]
    #[serde(rename = "WORLD_SURFACE")]
    pub world_surface: Vec<i64>,
}

impl HeightMaps {
    pub fn new() -> HeightMaps {
        HeightMaps {
            motion_blocking: Vec::from([0; 37]),
            world_surface: Vec::from([0; 37]),
        }
    }
}

impl Encoder for HeightMaps {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_network_nbt(self)?;
        Ok(())
    }
}

#[derive(derive_more::Deref)]
pub struct LongArray<const LEN: usize>([i64; LEN]);

impl<const LEN: usize> LongArray<LEN> {
    pub fn new() -> LongArray<LEN> {
        LongArray([0; LEN])
    }
}

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

impl<const LEN: usize> ByteArray<LEN> {
    pub fn new() -> ByteArray<LEN> {
        ByteArray([0; LEN])
    }
}

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

#[derive(Serialize, Deserialize, Debug)]
struct BlockEntityNbt {}
