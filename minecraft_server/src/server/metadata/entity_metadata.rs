use std::io::Result;

use crate::io::prelude::{Buffer, Encoder, Identifier, U8Write, VarIntWrite};
use derive_more::{Deref, From, Into};
use dyn_clone::DynClone;
use uuid::Uuid;

#[derive(Deref, From, Into)]
pub struct EntityMetadata(Vec<Option<Box<dyn EntityMeta>>>);

pub trait EntityMeta: Encoder {
    fn get_metadata_type_id(&self) -> i32;
}

impl Encoder for EntityMetadata {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        let mut i = 0;
        for metadata in self.iter() {
            if let Some(metadata) = metadata {
                buf.write_u8(i)?;
                buf.write_var_i32(metadata.get_metadata_type_id())?;
                metadata.encode_to_buffer(buf)?;
            }
            i += 1;
        }
        buf.write_u8(0xff)?;
        Ok(())
    }
}

impl EntityMetadata {
    pub fn new() -> EntityMetadata {
        EntityMetadata(vec![])
    }
}

impl Default for EntityMetadata {
    fn default() -> Self {
        Self(vec![])
    }
}
