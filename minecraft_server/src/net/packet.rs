use std::{
    io::{Read, Result, Write},
    ops::Deref,
};

use crate::io::prelude::{Decoder, Encoder};

use super::{prelude::Socket, server::Server};

pub trait PacketId: Sized {
    const PACKET_ID: i32;
}

pub trait PacketWriter<Player>: PacketId + Encoder {
    #[inline]
    fn send_packet(&self, player: &mut Socket<Player>) -> Result<()> {
        println!("[Sent]: {:?}", self);
        Socket::send_packet(player, self)?;
        Ok(())
    }
}

impl<P: Encoder + PacketId, Player> PacketWriter<Player> for P {}

pub trait PacketHandler<S: Server> {
    #[inline]
    fn handle_packet(&self, server: &mut S, player: &mut Socket<S::Player>) -> Result<()>;
}

pub trait PacketReadHandler<S: Server> {
    #[inline]
    fn handle_packet_read(server: &mut S, player: &mut Socket<S::Player>) -> Result<()>;
}
