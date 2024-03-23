use std::io::Result;

use crate::{
    io::prelude::{Buffer, Decoder, Encoder},
    net::prelude::{PacketHandler, Socket},
    server::prelude::{GamePlayer, GameServer},
};

pub struct AcknowledgeFinishConfiguration {}

impl Encoder for AcknowledgeFinishConfiguration {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        Ok(())
    }
}

impl Decoder for AcknowledgeFinishConfiguration {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(AcknowledgeFinishConfiguration {})
    }
}

impl PacketHandler<GameServer> for AcknowledgeFinishConfiguration {
    fn handle_packet(
        &self,
        server: &mut GameServer,
        player: &mut Socket<GamePlayer>,
    ) -> Result<()> {
        println!("AcknowledgeFinishConfiguration");
        Ok(())
    }
}
