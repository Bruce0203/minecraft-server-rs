use std::io::{Error, Result};

use bytes::{Buf, Bytes, BytesMut};
use common_server::{encoding::Decoder, var_string::VarStringRead, packet::PacketHandler};

use crate::{server::Server, prelude::Player};

#[derive(Debug)]
pub struct PluginMessage {
    channel: String,
    data: Bytes,
}

impl TryFrom<&mut BytesMut> for PluginMessage {
    type Error = Error;

    fn try_from(value: &mut BytesMut) -> Result<Self> {
        Ok(PluginMessage {
            channel: value.reader().read_var_string::<32767>()?,
            data: value.clone().freeze(),
        })
    }
}

impl PacketHandler<Server, Player> for PluginMessage {
    fn handle_packet(&self, server: &mut Server, player: &mut Player) -> Result<()> {
        Ok(())
    }
}
