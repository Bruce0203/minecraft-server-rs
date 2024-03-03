use std::io::{prelude::Write, Result};

use mc_io::{encoding::Encoder, primitives::F32Write};

use crate::{connection::prelude::PacketWriter, server::coordinates::Position};

pub struct SetDefaultPosition {
    pub location: Position,
    pub angle: f32,
}

impl Encoder for SetDefaultPosition {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.location.encode_to_write(writer)?;
        writer.write_f32(self.angle)?;
        Ok(())
    }
}

impl PacketWriter for SetDefaultPosition {
    fn get_packet_id(&self, player: &mut crate::server::prelude::Player) -> Result<i32> {
        Ok(0x54)
    }
}
