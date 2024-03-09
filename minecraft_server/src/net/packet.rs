use std::io::Result;

use crate::io::prelude::{Encoder, Decoder};

use super::{socket::Socket, prelude::Server};

pub trait PacketId<Player>: Sized {
    #[inline]
    fn get_packet_id(&self, player: &mut Socket<Player>) -> Result<i32>;
}

pub trait PacketWriter<Player>: PacketId<Player> + Encoder {
    #[inline]
    fn send_packet(&self, player: &mut Socket<Player>) -> Result<()> {
        Socket::send_packet(player, self)?;
        Ok(())
    }
}

impl<P: Encoder + PacketId<T>, T> PacketWriter<T> for P {}

pub trait PacketHandler<Server: super::prelude::Server> {
    #[inline]
    fn handle_packet(&self, server: &mut Server, player: &mut Socket<Server::Player>)
        -> Result<()>;
}

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
