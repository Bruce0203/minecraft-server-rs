use std::io::Cursor;
use std::io::Read;
use std::io::Result;
use std::io::Write;
use std::ops::Deref;

use super::buffer::Buffer;

pub trait Encoder {
    #[inline]
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()>;

    #[inline]
    fn encode(&self) -> Result<Cursor<Vec<u8>>> {
        let mut buf = Buffer::new(vec![]);
        self.encode_to_buffer(&mut buf)?;
        Ok(buf)
    }
}

pub trait Decoder: Sized {
    #[inline]
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self>;
}

pub auto trait EncoderDeref {}

pub auto trait DecoderDeref {}

impl<D: DecoderDeref + Deref<Target = T> + From<T>, T: Sized + Decoder> Decoder for D {
    fn decode_from_read<R: Read>(reader: &mut R) -> std::io::Result<D> {
        Ok(T::decode_from_read(reader)?.into())
    }
}

impl<D: EncoderDeref + Deref<Target = T>, T: Sized + Encoder> Encoder for D {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        T::encode_to_buffer(self, buf)
    }
}
