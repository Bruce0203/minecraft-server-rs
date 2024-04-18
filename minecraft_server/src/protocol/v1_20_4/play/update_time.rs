use std::io::Result;

use crate::{
    io::prelude::{Buffer, Decoder, Encoder, I64Read, I64Write},
    net::prelude::{PacketId, Socket},
    server::prelude::GamePlayer,
};

#[derive(Debug)]
pub struct UpdateTime {
    pub world_age: i64,
    pub time_of_day: i64,
}

impl Encoder for UpdateTime {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_i64(self.world_age)?;
        buf.write_i64(self.time_of_day)?;
        Ok(())
    }
}

impl Decoder for UpdateTime {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(UpdateTime {
            world_age: reader.read_i64()?,
            time_of_day: reader.read_i64()?,
        })
    }
}
