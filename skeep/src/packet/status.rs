use std::io::Error;
use std::io::Result;
use std::io::Write;

use bytes::Buf;
use bytes::BytesMut;
use common_server::packet::PacketHandler;
use common_server::var_int::VarIntRead;
use common_server::var_string::VarStringRead;

use crate::Player;

pub struct StatusRequest {}

impl StatusRequest {
    pub fn new() -> StatusRequest {
        StatusRequest {  }
    }
}

pub struct StatusResponse {
    name: String,
    description: String,
    max_players: i32,
    online_players: i32,
    has_password: bool,
}

impl TryFrom<BytesMut> for StatusResponse {
    type Error = Error;

    fn try_from(value: BytesMut) -> Result<Self> {
        let mut reader = value.reader();
        Ok(StatusResponse {
            name: reader.read_var_string::<256>()?,
            description: reader.read_var_string::<256>()?,
            max_players: reader.read_var_i32()?,
            online_players: reader.read_var_i32()?,
            has_password: unsafe { std::mem::transmute::<u8, bool>(reader.into_inner().get_u8()) },
        })
    }
}

impl PacketHandler<Player> for StatusRequest {
    fn handle_packet(&self, value: &mut common_server::selector::Socket<Player>) {
    }
}
