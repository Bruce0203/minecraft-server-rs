use std::{
    io::{Read, Result, Write},
    ops::Deref,
};

use crate::io::prelude::{Decoder, Encoder};

use super::{prelude::Server, socket::Socket};

pub trait PacketId: Sized {
    const PACKET_ID: i32;
}

pub trait PacketWriter<Player>: PacketId + Encoder {
    #[inline]
    fn send_packet(&self, player: &mut Socket<Player>) -> Result<()> {
        Socket::send_packet(player, self)?;
        Ok(())
    }
}

impl<P: Encoder + PacketId, T> PacketWriter<T> for P {}

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

impl<D: Deref<Target = T> + PacketId, T: Sized + Encoder> Encoder for D {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        T::encode_to_write(self, writer)
    }
}

impl<D: Deref<Target = T> + PacketId + From<T>, T: Sized + Decoder> Decoder for D {
    fn decode_from_read<R: Read>(reader: &mut R) -> std::io::Result<D> {
        Ok(T::decode_from_read(reader)?.into())
    }
}

impl<D: Deref<Target = T>, T: Sized + PacketHandler<S>, S: super::prelude::Server> PacketHandler<S>
    for D
{
    fn handle_packet(
        &self,
        server: &mut S,
        player: &mut Socket<<S as Server>::Player>,
    ) -> Result<()> {
        T::handle_packet(self, server, player)
    }
}
