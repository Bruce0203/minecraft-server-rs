use bytes::{BytesMut, Buf};
use common_server::{selector::Socket, var_int::VarIntRead};
use std::io::Result;

use crate::player::Player;

pub mod handshake;
pub mod status;

pub fn handle_packet(socket: &mut Socket<Player>, bytes: BytesMut) -> Result<()> {
    let mut reader = bytes.reader();
    let packet_id = reader.read_var_i32()?;
    Ok(())
}
