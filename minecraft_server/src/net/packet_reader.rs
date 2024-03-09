use std::io::Result;

use crate::io::prelude::Decoder;

use super::prelude::{PacketHandler, Socket, Server};

pub trait PacketReadHandler<S: Server> {
    #[inline]
    fn handle_packet_read(server: &mut S, player: &mut Socket<S::Player>) -> Result<()>;
}

impl<S: Server, P: PacketHandler<S> + Decoder> PacketReadHandler<S> for P {
    fn handle_packet_read(server: &mut S, player: &mut Socket<S::Player>) -> Result<()> {
        P::decode_from_read(&mut player.packet_buf)?.handle_packet(server, player)?;
        Ok(())
    }
}
