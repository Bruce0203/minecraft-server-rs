use std::io::Cursor;
use std::io::Read;
use std::io::Result;
use std::io::Write;

pub trait Encoder {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()>;

    fn encode(&self) -> Result<Cursor<Vec<u8>>> {
        let mut bytes = Cursor::new(Vec::new());
        self.encode_to_write(&mut bytes)?;
        Ok(bytes)
    }
}

pub trait Decoder: Sized {
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self>;
}
