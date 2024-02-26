use std::io::Result;
use std::io::{Error, Write};

use bytes::{Buf, BufMut, BytesMut};

pub trait Encoder {
    fn encode_to_bytes(&self, bytes: &mut BytesMut);

    fn encode(&self) -> BytesMut {
        let mut bytes = BytesMut::new();
        self.encode_to_bytes(&mut bytes);
        bytes
    }
}

pub trait Decoder: Sized {
    fn decode_to_bytes(value: BytesMut) -> Result<Self>;
}


