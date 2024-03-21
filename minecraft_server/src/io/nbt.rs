use std::io::{Cursor, Result};

use serde::{de::DeserializeOwned, Serialize};

pub trait NbtNetworkWrite {
    fn write_network_nbt<S: Serialize>(&mut self, value: &S) -> Result<()>;
}

impl<W: std::io::Write> NbtNetworkWrite for W {
    fn write_network_nbt<S: Serialize>(&mut self, value: &S) -> Result<()> {
        let mut buf = Cursor::new(Vec::new());
        nbt::to_writer(&mut buf, value, None)?;
        self.write_all(&[10])?;
        self.write_all(&buf.get_ref()[3..])?;
        Ok(())
    }
}

pub trait NbtNetworkRead {
    fn read_network_nbt<D: DeserializeOwned>(&mut self) -> Result<D>;
}

impl<R: std::io::Read> NbtNetworkRead for R {
    fn read_network_nbt<D: DeserializeOwned>(&mut self) -> Result<D> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        assert_eq!(buf[0], 10);
        Ok(nbt::from_reader(self)?)
    }
}
