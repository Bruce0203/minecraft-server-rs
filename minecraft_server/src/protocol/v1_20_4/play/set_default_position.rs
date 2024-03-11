use std::io::{prelude::Write, Result};

use crate::io::prelude::{Encoder, F32Write};
use crate::net::prelude::{PacketId, Socket};
use crate::server::coordinates::Position;
use crate::server::prelude::GamePlayer;

pub struct SetDefaultPosition {
    pub location: Position,
    pub angle: f32,
}

impl Encoder for SetDefaultPosition {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        self.location.encode_to_buffer(buf)?;
        buf.write_f32(self.angle)?;
        Ok(())
    }
}
