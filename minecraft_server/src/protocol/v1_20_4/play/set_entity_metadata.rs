use std::{
    fs::Metadata,
    io::{prelude::Write, Result},
};

use uuid::Uuid;

use crate::{
    io::prelude::{
        Decoder, Encoder, F32Write, I64Write, Identifier, U8Read, U8Write, VarIntRead,
        VarIntSizedVecRead, VarIntWrite, VarStringWrite,
    },
    protocol::v1_20_4::configuration::registry::Particle,
    server::{
        chat::ChatNbtWrite, coordinates::{Direction, Position}, prelude::EntityMeta, slot::Slot
    },
};

pub struct SetEntityMetadata {
    pub entity_id: i32,
    pub metadata: EntityMeta,
}

impl Decoder for SetEntityMetadata {
    fn decode_from_read<R: std::io::prelude::Read>(reader: &mut R) -> Result<Self> {
        Ok(SetEntityMetadata {
            entity_id: reader.read_var_i32()?,
            metadata: EntityMeta::default(),
        })
    }
}

impl Encoder for SetEntityMetadata {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        buf.write_var_i32(self.entity_id)?;
        self.metadata.encode_to_buffer(buf)?;
        buf.write_u8(0xff)?;
        Ok(())
    }
}
