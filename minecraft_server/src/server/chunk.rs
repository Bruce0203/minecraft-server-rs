use std::{
    char::MAX,
    io::{Result, Seek, Write},
};

use nbt::Map;
use serde::{Deserialize, Serialize};

use crate::io::prelude::{
    BitSet, BitSetWrite, Buffer, Decoder, Encoder, I16Read, I16Write, I32Read, I32Write,
    NbtNetworkRead, NbtNetworkWrite, U16Write, U8Read, U8Write, VarIntRead, VarIntSizedVecRead,
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

impl Decoder for Chunk {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(Chunk {
            x: reader.read_i32()?,
            z: reader.read_i32()?,
            heightmaps: HeightMaps::decode_from_read(reader)?,
            sections: reader.read_var_int_sized_vec()?,
            block_entities: reader.read_var_int_sized_vec()?,
            light: Light::decode_from_read(reader)?,
        })
    }
}

#[test]
fn test_chunk() {
    let chunk = Chunk::new(0, 0);
    println!("{:?}", *chunk.sections.encode().unwrap().get_ref());
    println!("{:?}", chunk.sections.encode().unwrap().get_ref().len());
}

#[derive(Debug)]
pub struct ChunkSection {
    block_states: Palette,
    biomes: Palette,
}

impl Encoder for Vec<ChunkSection> {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        //length is not encoded
        for chunk_section in self.iter() {
            chunk_section.block_states.encode_to_buffer(buf)?;
            chunk_section.biomes.encode_to_buffer(buf)?;
        }
        Ok(())
    }
}

impl Decoder for Vec<ChunkSection> {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        let mut vec = Vec::new();
        while !reader.is_empty() {
            vec.push(ChunkSection::decode_from_read(reader)?)
        }
        Ok(vec)
    }
}

impl Decoder for ChunkSection {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(ChunkSection {
            block_states: Palette::decode_from_read(reader)?,
            biomes: Palette::decode_from_read(reader)?,
        })
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

pub struct HeightMaps {
    nbt: nbt::Value,
}

impl HeightMaps {
    pub fn new() -> HeightMaps {
        HeightMaps {
            nbt: nbt::Value::Compound(Map::new()),
        }
    }
}

impl Encoder for HeightMaps {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_nbt_compound(&self.nbt)?;
        Ok(())
    }
}

impl Decoder for HeightMaps {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(HeightMaps {
            nbt: reader.read_nbt_compound()?,
        })
    }
}

#[derive(derive_more::Deref)]
pub struct LongArray<const LEN: usize>([i64; LEN]);

impl<const LEN: usize> LongArray<LEN> {
    pub fn new() -> LongArray<LEN> {
        LongArray([0; LEN])
    }
}

pub struct ByteArray<const LEN: usize>([u8; LEN]);

impl<const LEN: usize> ByteArray<LEN> {
    pub fn new() -> ByteArray<LEN> {
        ByteArray([0; LEN])
    }
}

pub struct BlockEntity {
    x: u8,
    y: i16,
    z: u8,
    entity_type: i32,
    data: nbt::Value,
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

impl Decoder for BlockEntity {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        let packed_xz = reader.read_u8()?;
        Ok(BlockEntity {
            x: packed_xz >> 4,
            z: packed_xz & 15,
            y: reader.read_i16()?,
            entity_type: reader.read_var_i32()?,
            data: reader.read_nbt_compound()?,
        })
    }
}

