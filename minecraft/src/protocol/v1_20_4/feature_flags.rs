use std::io::{Error, Result, prelude::Write, Cursor};

use common_server::{array::{VarIntSizedVecRead, VarIntSizedVecWrite}, identifier::Identifier, encoding::Encoder};

use crate::connection::{packet_writer::PacketWriter, player::Player};

pub struct FeatureFlags {
    pub feature_flags: Vec<Identifier>,
}

impl TryFrom<&mut Cursor<Vec<u8>>> for FeatureFlags {
    type Error = Error;

    fn try_from(value: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(FeatureFlags {
            feature_flags: value.read_var_int_sized_vec()?,
        })
    }
}

impl Encoder for FeatureFlags {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_var_int_sized_vec(&self.feature_flags)?;
        Ok(())
    }
}

impl PacketWriter for FeatureFlags {
    fn get_packet_id(&self, player: &mut Player) -> Result<i32> {
        Ok((0x08))
    }
}
