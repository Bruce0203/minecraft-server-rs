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
        chat::ChatNbtWrite,
        coordinates::{Direction, Position},
        prelude::{Chat, EntityMeta, EntityMetadata, Player},
        slot::Slot,
    },
};

pub struct SetEntityMetadata {
    pub entity_id: i32,
    pub metadata: Box<dyn crate::server::prelude::Metadata>,
}

impl Decoder for SetEntityMetadata {
    fn decode_from_read<R: std::io::prelude::Read>(reader: &mut R) -> Result<Self> {
        Ok(SetEntityMetadata {
            entity_id: reader.read_var_i32()?,
            metadata: Box::new(Player::default()),
        })
    }
}

impl Encoder for SetEntityMetadata {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_var_i32(self.entity_id)?;
        Ok(())
    }
}
