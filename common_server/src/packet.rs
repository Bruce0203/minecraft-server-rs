use std::io::{Result, Write};

use bytes::{BytesMut, BufMut};

use crate::{encoder::Encoder, selector::Socket, var_int::VarIntWrite};

pub trait PacketHandler<Server, Player> {
    fn handle_packet(&self, server: &mut Server, socket: &mut Socket<Player>) -> Result<()>;
}

pub trait PacketWriter<T>: Sized + Encoder {
    fn get_packet_id(&self, socket: &mut Socket<T>) -> Result<i32>;

    fn send_packet(&self, socket: &mut Socket<T>) -> Result<()> {
        let payload = self.encode()?; 
        let packet_id = self.get_packet_id(socket)?;
        let buf = BytesMut::with_capacity(payload.len() + 5);
        let mut writer = buf.writer();
        let packet_id_data_len = writer.write_var_i32(packet_id)?;
        writer.write(&payload)?;
        socket.stream.write_var_i32((payload.len() + packet_id_data_len) as i32)?;
        socket.stream.write_all(&writer.into_inner())?;
        Ok(())
    }
}
