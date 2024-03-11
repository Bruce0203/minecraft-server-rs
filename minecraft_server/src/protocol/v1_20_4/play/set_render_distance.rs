use std::io::Result;

use crate::io::prelude::Encoder;
use crate::io::prelude::VarIntWrite;
use crate::net::prelude::PacketId;
use crate::net::prelude::Socket;
use crate::server::prelude::GamePlayer;

use super::set_default_position::SetDefaultPosition;

pub struct SetRenderDistance {
    pub view_distance: i32,
}

impl Encoder for SetRenderDistance {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        buf.write_var_i32(self.view_distance);
        Ok(())
    }
}
