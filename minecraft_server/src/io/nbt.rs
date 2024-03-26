use std::{
    fmt::Debug,
    io::{Cursor, Error, ErrorKind, Read, Result, Write},
};

use serde::{de::DeserializeOwned, Serialize};
use simdnbt::borrow::NbtCompound;

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
    fn read_network_nbt<'a, D: TryFrom<NbtCompound<'a>, Error = Error>>(&'a mut self) -> Result<D>;
}

impl NbtNetworkRead for Buffer {
    fn read_network_nbt<'a, D: TryFrom<NbtCompound<'a>, Error = Error>>(&'a mut self) -> Result<D> {
        let mut buf = Cursor::new(&self.get_ref()[..]);
        buf.set_position(self.position());
        let mut value = match NbtCompound::read(&mut buf) {
            Ok(value) => value,
            Err(err) => Err(Error::new(ErrorKind::InvalidInput, err))?,
        };
        Ok(D::try_from(value)?)
    }
}
