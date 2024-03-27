use std::io::Result;

use crate::{
    io::prelude::{Buffer, Decoder, Encoder},
    net::prelude::{PacketHandler, Socket},
    server::prelude::{GamePlayer, GameServer},
};

#[derive(Debug)]
pub struct UpdateReceipes {
    //TODO wip 
}

impl Encoder for UpdateReceipes {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        Ok(())
    }
}

impl Decoder for UpdateReceipes {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(UpdateReceipes {})
    }
}

impl PacketHandler<GameServer> for UpdateReceipes {
    fn handle_packet(
        &self,
        server: &mut GameServer,
        player: &mut Socket<GamePlayer>,
    ) -> Result<()> {
        println!("update receipe: {:?}", self);
        Ok(())
    }
}
