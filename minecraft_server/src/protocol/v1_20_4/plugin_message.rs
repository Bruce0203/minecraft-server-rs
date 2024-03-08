use std::io::{BufRead, Cursor, Error, Read, Result};

use crate::io::prelude::{VarStringRead, Decoder};
use crate::net::prelude::{PacketHandler, Player};
use crate::server::prelude::Server;

#[derive(Debug)]
pub struct PluginMessage {
    channel: String,
    data: Vec<u8>,
}

impl Decoder for PluginMessage {
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self> {
        let channel = reader.read_var_string::<32767>()?;
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        Ok(PluginMessage { channel, data })
    }
}

impl PacketHandler for PluginMessage {
    fn handle_packet(&self, server: &mut Server, _player: &mut Player) -> Result<()> {
        Ok(())
    }
}
