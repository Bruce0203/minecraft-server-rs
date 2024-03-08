use std::io::{Result, Write};

use crate::{
    io::prelude::{Encoder, VarIntWrite},
    net::prelude::{PacketIdentifier, Player},
};

pub struct SetSimulationDistance {
    pub simulation_distance: i32,
}

impl Encoder for SetSimulationDistance {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_var_i32(self.simulation_distance)?;
        Ok(())
    }
}

impl PacketIdentifier for SetSimulationDistance {
    fn get_packet_id(&self, player: &mut Player) -> Result<i32> {
        Ok(0x60)
    }
}
