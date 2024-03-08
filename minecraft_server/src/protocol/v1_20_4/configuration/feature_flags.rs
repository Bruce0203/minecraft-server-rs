use std::io::{prelude::Write, Cursor, Error, Result};

use crate::io::prelude::Encoder;
use crate::io::prelude::Identifier;
use crate::io::prelude::VarIntSizedVecRead;
use crate::io::prelude::VarIntSizedVecWrite;
use crate::net::prelude::LoginPlayer;
use crate::net::prelude::Player;

use crate::net::prelude::PacketIdentifier;

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

impl PacketIdentifier<LoginPlayer> for FeatureFlags {
    fn get_packet_id(&self, _player: &mut LoginPlayer) -> Result<i32> {
        Ok(0x08)
    }
}
