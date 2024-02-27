use std::io::Result;
use std::io::{Error, Write};

use bytes::{Buf, BufMut, BytesMut};

use crate::selector::Socket;

pub trait Encoder {
    fn encode_to_write<W: Write>(&self, writer: &mut W);

    fn encode(&self) -> BytesMut {
        let mut bytes = BytesMut::new().writer();
        self.encode_to_write(&mut bytes);
        bytes.into_inner()
    }
}

pub trait PacketWriter<T>: Sized + Encoder {
    fn send_packet(&self, socket: &mut Socket<T>) -> Result<()>;
}

