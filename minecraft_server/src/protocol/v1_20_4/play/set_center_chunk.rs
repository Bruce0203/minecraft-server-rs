use std::io::{Result, Write};

use crate::io::prelude::VarIntWrite;
use crate::net::prelude::Socket;
use crate::server::prelude::GamePlayer;
use crate::{io::prelude::Encoder, net::prelude::PacketId};

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

