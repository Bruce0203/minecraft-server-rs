use std::io::Cursor;

use serde::{de::DeserializeOwned, Serialize};

pub trait NbtNetworkWrite {
    fn write_network_nbt<S: Serialize>(&mut self, value: &S) -> std::io::Result<()>;
}

impl<W: std::io::Write> NbtNetworkWrite for W {
    fn write_network_nbt<S: Serialize>(&mut self, value: &S) -> std::io::Result<()> {
        let mut buf = Cursor::new(Vec::new());
        nbt::to_writer(&mut buf, value, Some(""))?;
        self.write_all(&[10])?;
        self.write_all(&buf.get_ref()[3..])?;
        Ok(())
    }
}

pub trait NbtNetworkRead {
    fn read_network_nbt<D: DeserializeOwned>(&mut self) -> std::io::Result<D>;
}

impl<R: std::io::Read> NbtNetworkRead for R {
    fn read_network_nbt<D: DeserializeOwned>(&mut self) -> std::io::Result<D> {
        Ok(nbt::from_reader(self)?)
    }
}
