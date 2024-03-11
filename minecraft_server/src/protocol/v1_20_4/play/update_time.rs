use crate::{
    io::prelude::{Encoder, I64Write},
    net::prelude::{PacketId, Socket},
    server::prelude::GamePlayer,
};

pub struct UpdateTime {
    pub world_age: i64,
    pub time_of_day: i64,
}

impl Encoder for UpdateTime {
    fn encode_to_write<W: std::io::prelude::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_i64(self.world_age)?;
        writer.write_i64(self.time_of_day)?;
        Ok(())
    }
}

