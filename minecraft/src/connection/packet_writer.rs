use std::io::{Result, Write};

use bytes::{BufMut, BytesMut};
use common_server::{encoding::Encoder, selector::Socket, var_int::VarIntWrite};

use crate::prelude::Player;

pub trait PacketWriter: Sized + Encoder {
    fn get_packet_id(&self, player: &mut Player) -> Result<i32>;

    fn send_packet(&self, player: &mut Player) -> Result<()> {
        let payload = self.encode()?;
        let packet_id = self.get_packet_id(player)?;
        let payload_buf = BytesMut::with_capacity(payload.len() + 5);
        let mut payload_writer = payload_buf.writer();
        let packet_id_data_len = payload_writer.write_var_i32(packet_id)?;
        payload_writer.write(&payload)?;
        let write_buffer = &mut player.write_buffer;
        let payload_data_len = (payload.len() + packet_id_data_len) as i32;
        let final_buffer = &mut BytesMut::with_capacity(payload_data_len as usize + 5);
        final_buffer.writer().write_var_i32(payload_data_len)?;
        final_buffer.writer().write_all(&payload_writer.into_inner())?;
        write_buffer.writer().write_all(&final_buffer)?;
        Ok(())
    }
}
