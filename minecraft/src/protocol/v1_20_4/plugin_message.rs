use std::io::{Error, Result};

use bytes::{Buf, Bytes, BytesMut};
use common_server::{encoding::Decoder, packet::PacketHandler, var_string::VarStringRead};

use crate::{server::Server, connection::player::Player};

#[derive(Debug)]
pub struct PluginMessage {
    channel: String,
    data: Bytes,
}

impl TryFrom<&mut BytesMut> for PluginMessage {
    type Error = Error;

    fn try_from(value: &mut BytesMut) -> Result<Self> {
        let channel = value.reader().read_var_string::<32767>()?;
        let data = value.clone().freeze();
        value.clear();
        Ok(PluginMessage { channel, data })
    }
}

impl PacketHandler<Server, Player> for PluginMessage {
    fn handle_packet(&self, _server: &mut Server, _player: &mut Player) -> Result<()> {
        Ok(())
    }
}
