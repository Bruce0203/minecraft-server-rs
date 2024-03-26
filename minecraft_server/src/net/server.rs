use std::io::{Cursor, Error, ErrorKind, Read, Result, Write};

use flate2::write::ZlibDecoder;

use crate::io::prelude::{Decoder, VarIntRead};

use super::{prelude::PacketHandler, selector::SelectorUpdateListener, socket::Socket};

pub trait MaxPacketBufferSize {
    const MAX_PACKET_BUFFER_SIZE: usize;
}

pub trait Server: Sized + SelectorUpdateListener<Self> + MaxPacketBufferSize {
    type Player: Default;

    fn read_packet(&mut self, player: &mut Socket<Self::Player>) -> Result<()>;
}
