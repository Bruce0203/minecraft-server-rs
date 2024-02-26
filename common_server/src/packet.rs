use std::io::Write;

use crate::{encoder::Encoder, selector::Socket, var_int::VarIntWrite};
use bytes::{BufMut, BytesMut};

pub trait PacketHandler<P, S> {
    fn handle_packet(&self, server: &mut S, player: &mut Socket<P>);
}


pub trait PacketIdentifier {
    fn get_id() -> i32;
}

impl<S> Socket<S> {
    fn send_packet<T: Encoder + PacketIdentifier>(&mut self, mut value: T) {
        let mut buf = BytesMut::new();
        let id = T::get_id();
        let mut writer = buf.writer();
        let payload = value.encode();
        writer.write_var_i32(payload.len() as i32).unwrap();
        self.stream.write_all(writer.into_inner().as_mut()).unwrap();
    }
}

