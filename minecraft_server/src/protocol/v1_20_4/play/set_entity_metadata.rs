use std::{
    fs::Metadata,
    io::{prelude::Write, Result},
};

use uuid::Uuid;

use crate::{
    io::prelude::{
        Buffer, Decoder, Encoder, F32Write, I64Write, Identifier, U8Read, U8Write, VarIntRead,
        VarIntSizedVecRead, VarIntWrite, VarStringWrite,
    },
    protocol::v1_20_4::configuration::registry::Particle,
    server::{
        chat::ChatNbtWrite,
        coordinates::{Direction, Position},
        metadata::prelude::EntityMetadata,
        prelude::EntityMeta,
        slot::Slot,
    },
};

#[derive(Debug)]
pub struct SetEntityMetadata {
    pub entity_id: i32,
    pub metadata: EntityMetadata,
}

impl Decoder for SetEntityMetadata {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(SetEntityMetadata {
            entity_id: reader.read_var_i32()?,
            metadata: EntityMetadata(Box::new(EntityMeta::default())),
        })
    }
}

impl Encoder for SetEntityMetadata {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        buf.write_var_i32(self.entity_id)?;
        self.metadata.encode_to_buffer(buf)?;
        Ok(())
    }
}
