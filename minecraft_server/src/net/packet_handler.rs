use std::io::Result;

use crate::{server::prelude::LoginServer, io::prelude::Decoder};

use super::prelude::Socket;

pub trait PacketHandler<Server: super::prelude::Server> {
    #[inline]
    fn handle_packet(&self, server: &mut Server, player: &mut Socket<Server::Player>) -> Result<()>;
}

