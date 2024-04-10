use core::panic;
use std::{
    fmt::Debug,
    io::{Cursor, Read, Result, Write},
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::io::prelude::U8Read;

use super::buffer::Buffer;

pub trait NbtNetworkWrite {
    fn write_network_nbt<S: Serialize + Debug>(&mut self, value: &S) -> Result<()>;
    fn write_nbt_compound(&mut self, value: &nbt::Value) -> Result<()>;
}

impl NbtNetworkWrite for Buffer {
    fn write_network_nbt<S: Serialize + Debug>(&mut self, value: &S) -> Result<()> {
        let mut buf = Cursor::new(Vec::new());
        nbt::to_writer(&mut buf, value, None)?;
        self.write_all(&[10])?;
        self.write_all(&buf.get_ref()[3..])?;
        Ok(())
    }

    fn write_nbt_compound(&mut self, value: &nbt::Value) -> Result<()> {
        Ok(())
    }
}

pub trait NbtNetworkRead {
    fn read_network_nbt<D: DeserializeOwned + Debug>(&mut self) -> Result<D>;
    fn read_nbt_compound(&mut self) -> Result<nbt::Value>;
}

impl NbtNetworkRead for Buffer {
    fn read_network_nbt<D: DeserializeOwned + Debug>(&mut self) -> Result<D> {
        let pos = self.position();
        self.set_position(pos + 1);
        let value = Ok(nbt::de::from_reader(&mut *self)?);
        self.set_position(self.position() + 1);
        value
    }

    fn read_nbt_compound(&mut self) -> Result<nbt::Value> {
        let pos = self.position();
        self.set_position(pos + 1);
        let value = nbt::Value::from_reader(10, &mut *self)?;
        self.set_position(self.position() + 1);
        Ok(value)
    }
}
