use std::{
    fmt::Debug,
    io::{Cursor, Read, Result, Write},
};

use serde::{de::DeserializeOwned, Serialize};

use super::buffer::Buffer;

pub trait NbtNetworkWrite {
    fn write_network_nbt<S: Serialize + Debug>(&mut self, value: &S) -> Result<()>;
}

impl NbtNetworkWrite for Buffer {
    fn write_network_nbt<S: Serialize + Debug>(&mut self, value: &S) -> Result<()> {
        let mut buf = Cursor::new(Vec::new());
        nbt::to_writer(&mut buf, value, None)?;
        self.write_all(&[10])?;
        self.write_all(&buf.get_ref()[3..])?;

        Ok(())
    }
}

pub trait NbtNetworkRead {
    fn read_network_nbt<D: DeserializeOwned + Debug>(&mut self) -> Result<D>;
}

impl NbtNetworkRead for Buffer {
    fn read_network_nbt<D: DeserializeOwned + Debug>(&mut self) -> Result<D> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        assert_eq!(buf[0], 10);
        let mut cloned = self.clone();
        let value2 = nbt::Value::from_reader(10, &mut cloned);
        let value = nbt::from_reader(self)?;
        //println!("{:#?}", value2);
        Ok(value)
    }
}
