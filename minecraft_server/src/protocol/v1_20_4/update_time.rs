use crate::{
    io::prelude::{Encoder, I64Write},
    net::prelude::PacketIdentifier,
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

impl PacketIdentifier for UpdateTime {
    fn get_packet_id(&self, player: &mut crate::net::prelude::Player) -> std::io::Result<i32> {
        Ok(0x62)
    }
}
