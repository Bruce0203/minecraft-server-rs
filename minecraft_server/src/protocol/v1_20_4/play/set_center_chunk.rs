use std::io::{Result, Write};

use crate::io::prelude::{Decoder, VarIntRead, VarIntWrite};
use crate::net::prelude::Socket;
use crate::server::prelude::GamePlayer;
use crate::{io::prelude::Encoder, net::prelude::PacketId};

#[derive(Debug)]
pub struct SetCenterChunk {
    pub chunk_x: i32,
    pub chunk_z: i32,
}

impl Encoder for SetCenterChunk {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        buf.write_var_i32(self.chunk_x)?;
        buf.write_var_i32(self.chunk_z)?;
        Ok(())
    }
}

impl Decoder for SetCenterChunk {
    fn decode_from_read(reader: &mut crate::io::prelude::Buffer) -> Result<Self> {
        Ok(SetCenterChunk {
            chunk_x: reader.read_var_i32()?,
            chunk_z: reader.read_var_i32()?,
        })
    }
}
