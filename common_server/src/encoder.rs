use std::io::Result;
use std::io::Write;

use bytes::{BufMut, BytesMut};

pub trait Encoder {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()>;

    fn encode(&self) -> Result<BytesMut> {
        let mut bytes = BytesMut::new().writer();
        self.encode_to_write(&mut bytes)?;
        Ok(bytes.into_inner())
    }
}


