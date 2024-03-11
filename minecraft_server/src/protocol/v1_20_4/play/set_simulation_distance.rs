use std::io::{Result, Write};

use crate::{
    io::prelude::{Encoder, VarIntWrite},
    net::prelude::{PacketId, Socket},
    server::prelude::GamePlayer,
};

pub struct SetSimulationDistance {
    pub simulation_distance: i32,
}

impl Encoder for SetSimulationDistance {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        buf.write_var_i32(self.simulation_distance)?;
        Ok(())
    }
}

