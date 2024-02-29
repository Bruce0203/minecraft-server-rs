use std::io::Result;

use bytes::BytesMut;
use common_server::selector::Socket;

use crate::{player::Player, server::Server};

pub trait PacketReadHandler<'server> {
    fn handle_packet_read(
        server: &mut Server,
        socket: &mut Player,
        value: &mut BytesMut,
    ) -> Result<()>;
}
