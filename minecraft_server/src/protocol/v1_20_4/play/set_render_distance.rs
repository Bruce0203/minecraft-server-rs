use std::io::Result;

use crate::io::prelude::Buffer;
use crate::io::prelude::Decoder;
use crate::io::prelude::Encoder;
use crate::io::prelude::VarIntRead;
use crate::io::prelude::VarIntWrite;
use crate::net::prelude::PacketId;
use crate::net::prelude::Socket;
use crate::server::prelude::GamePlayer;

use super::set_default_position::SetDefaultPosition;

#[derive(Debug)]
pub struct SetRenderDistance {
    pub view_distance: i32,
}

impl Encoder for SetRenderDistance {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(self.view_distance);
        Ok(())
    }
}

impl Decoder for SetRenderDistance {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(SetRenderDistance {
            view_distance: reader.read_var_i32()?,
        })
    }
}
