use std::io::{Error, Result, Write};

use bytes::BytesMut;
use common_server::{packet::PacketHandler, encoding::Encoder};

use crate::{server::Server, connection::{player::Player, ConnectionState, packet_writer::PacketWriter}};

pub struct FinishConfiguration {}

impl FinishConfiguration {
    pub fn new() -> FinishConfiguration {
        FinishConfiguration {  }
    }
}

impl TryFrom<&mut BytesMut> for FinishConfiguration {
    type Error = Error;

    fn try_from(value: &mut BytesMut) -> Result<Self> {
        Ok(FinishConfiguration {})
    }
}

impl PacketHandler<Server, Player> for FinishConfiguration {
    fn handle_packet(&self, server: &mut Server, player: &mut Player) -> Result<()> {
        println!("configuration finished");
        player.session_relay.connection_state = ConnectionState::Play;
        Ok(())
    }
}

impl PacketWriter for FinishConfiguration {
    fn get_packet_id(&self, player: &mut Player) -> Result<i32> {
        Ok(0x02)
    }
}

impl Encoder for FinishConfiguration {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        Ok(())
    }
}
