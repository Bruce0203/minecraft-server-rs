use std::io::{prelude::Write, Cursor, Error, Result};

use crate::io::prelude::Encoder;
use crate::io::prelude::Identifier;
use crate::io::prelude::VarIntSizedVecRead;
use crate::io::prelude::VarIntSizedVecWrite;
use crate::net::prelude::PacketId;
use crate::net::prelude::Socket;
use crate::server::prelude::GamePlayer;

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
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        buf.write_var_int_sized_vec(&self.feature_flags)?;
        Ok(())
    }
}
