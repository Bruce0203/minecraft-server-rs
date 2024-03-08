use std::io::{Result, Write};

use crate::net::prelude::LoginPlayer;
use crate::{io::prelude::Encoder, net::prelude::PacketIdentifier};
use crate::io::prelude::VarIntWrite;

pub struct SetCenterChunk {
    pub chunk_x: i32,
    pub chunk_z: i32,
}

impl Encoder for SetCenterChunk {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_var_i32(self.chunk_x)?;
        writer.write_var_i32(self.chunk_z)?;
        Ok(())
    }
}

impl PacketIdentifier<LoginPlayer> for SetCenterChunk {
    fn get_packet_id(&self, player: &mut LoginPlayer) -> Result<i32> {
        Ok(0x52)
    }
}
