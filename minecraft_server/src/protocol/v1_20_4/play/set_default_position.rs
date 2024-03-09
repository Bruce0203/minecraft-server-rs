use std::io::{prelude::Write, Result};

use crate::io::prelude::{Encoder, F32Write};
use crate::net::prelude::{PacketIdentifier, Socket};
use crate::server::coordinates::Position;
use crate::server::prelude::LoginPlayer;

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

impl PacketIdentifier<LoginPlayer> for SetDefaultPosition {
    fn get_protocol_id(&self, player: &mut Socket<LoginPlayer>) -> Result<i32> {
        Ok(0x54)
    }
}
