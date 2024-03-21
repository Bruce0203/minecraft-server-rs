use std::io::{Cursor, Error, ErrorKind, Read, Result, Write};

use flate2::write::ZlibDecoder;

use crate::io::prelude::{Decoder, VarIntRead};

use super::{prelude::PacketHandler, socket::Socket};

pub trait Server: Sized {
    type Player: Default;

    fn read_packet(&mut self, player: &mut Socket<Self::Player>) -> Result<()>;
}
