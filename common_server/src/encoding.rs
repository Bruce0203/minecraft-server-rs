use std::io::Read;
use std::io::Result;
use std::io::Write;

use bytes::BufMut;
use bytes::BytesMut;

pub trait Encoder {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()>;

    fn encode(&self) -> Result<BytesMut> {
        let mut bytes = BytesMut::new().writer();
        self.encode_to_write(&mut bytes)?;
        Ok(bytes.into_inner())
    }
}

pub trait Decoder: Sized {
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self>;
}
