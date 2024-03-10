use std::io::{BufRead, Cursor, Error, Read, Result};

use crate::{
    io::prelude::{Decoder, VarStringRead},
    net::prelude::{PacketHandler, Socket},
    server::prelude::{LoginPlayer, LoginServer},
};

use derive_more::{Deref, From};

#[derive(Deref, From)]
pub struct C2SPluginMessageConfiguration(PluginMessage);

#[derive(Deref, From)]
pub struct C2SPluginMessagePlay(PluginMessage);

#[derive(Deref, From)]
pub struct S2CPluginMessageConfiguration(PluginMessage);

#[derive(Deref, From)]
pub struct S2CPluginMessagePlay(PluginMessage);

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

impl PacketHandler<LoginServer> for PluginMessage {
    fn handle_packet(
        &self,
        server: &mut LoginServer,
        _player: &mut Socket<LoginPlayer>,
    ) -> Result<()> {
        Ok(())
    }
}
