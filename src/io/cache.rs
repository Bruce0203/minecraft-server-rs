use std::{
    cell::RefCell,
    io::{prelude::Write, Result},
    mem::MaybeUninit,
    ops::Deref,
};

use super::prelude::Encoder;

pub struct Cache<T: Encoder> {
    inner: T,
    cache: RefCell<MaybeUninit<Vec<u8>>>,
    is_cached: RefCell<bool>,
}

impl<T: Encoder> From<T> for Cache<T> {
    fn from(value: T) -> Self {
        Cache {
            inner: value,
            cache: RefCell::new(MaybeUninit::uninit()),
            is_cached: RefCell::new(false),
        }
    }
}

impl<T: Encoder> Deref for Cache<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Encoder> Encoder for Cache<T> {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let mut is_cached = self.is_cached.borrow_mut();
        if *is_cached {
            writer.write_all(unsafe { self.cache.borrow_mut().assume_init_ref() })?;
        } else {
            let data = T::encode(self)?.into_inner();
            writer.write_all(&data)?;
            self.cache.borrow_mut().write(data);
            *is_cached = true;
        }
        Ok(())
    }
}
