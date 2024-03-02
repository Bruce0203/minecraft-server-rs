use std::io::{BufRead, Cursor, Error, Read, Result};

use mc_io::var_string::VarStringRead;

use crate::{connection::{player::Player, packet_handler::PacketHandler}, server::Server};

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

impl PacketHandler<Server, Player> for PluginMessage {
    fn handle_packet(&self, _server: &mut Server, _player: &mut Player) -> Result<()> {
        Ok(())
    }
}
