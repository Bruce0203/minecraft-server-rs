use std::io::{Result, Write};

use crate::{
    io::prelude::{Buffer, Decoder, Encoder, VarIntRead, VarIntWrite},
    net::prelude::{PacketId, Socket},
    server::prelude::GamePlayer,
};

#[derive(Debug)]
pub struct SetSimulationDistance {
    pub simulation_distance: i32,
}

impl Encoder for SetSimulationDistance {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(self.simulation_distance)?;
        Ok(())
    }
}

impl Decoder for SetSimulationDistance {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(SetSimulationDistance {
            simulation_distance: reader.read_var_i32()?,
        })
    }
}
