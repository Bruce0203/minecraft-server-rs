use std::io::{BufRead, Cursor, Error, Read, Result};

use crate::io::prelude::VarStringRead;
use crate::net::prelude::{PacketHandler, Player};
use crate::server::prelude::Server;

#[derive(Debug)]
pub struct PluginMessage {
    channel: String,
    data: Vec<u8>,
}

impl TryFrom<&mut Cursor<Vec<u8>>> for PluginMessage {
    type Error = Error;

    fn try_from(value: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let channel = value.read_var_string::<32767>()?;
        let mut data = Vec::<u8>::with_capacity(value.get_ref().len());
        value.read_to_end(&mut data);
        Ok(PluginMessage { channel, data })
    }
}

impl PacketHandler for PluginMessage {
    fn handle_packet(&self, server: &mut Server, _player: &mut Player) -> Result<()> {
        Ok(())
    }
}
