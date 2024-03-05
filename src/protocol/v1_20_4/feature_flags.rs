use std::io::{prelude::Write, Cursor, Error, Result};

use crate::io::{
    array::{VarIntSizedVecRead, VarIntSizedVecWrite},
    encoding::Encoder,
    identifier::Identifier,
};

use crate::protocol::prelude::PacketWriter;
use crate::server::prelude::Player;

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
    fn get_packet_id(&self, _player: &mut Player) -> Result<i32> {
        Ok(0x08)
    }
}
