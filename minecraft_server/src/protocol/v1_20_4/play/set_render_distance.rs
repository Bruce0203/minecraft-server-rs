use std::io::Result;

use crate::io::prelude::Encoder;
use crate::io::prelude::VarIntWrite;
use crate::net::prelude::PacketId;
use crate::net::prelude::Socket;
use crate::server::prelude::LoginPlayer;

use super::set_default_position::SetDefaultPosition;

pub struct SetRenderDistance {
    pub view_distance: i32,
}

impl Encoder for SetRenderDistance {
    fn encode_to_write<W: std::io::prelude::Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_var_i32(self.view_distance);
        Ok(())
    }
}

impl PacketId<LoginPlayer> for SetRenderDistance {
    fn get_packet_id(&self, player: &mut Socket<LoginPlayer>) -> Result<i32> {
        Ok(0x53)
    }
}
