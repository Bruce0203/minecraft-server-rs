use std::{
    cell::UnsafeCell,
    io::{prelude::Write, Result},
    ops::Deref,
};

use super::prelude::{DecoderDeref, Encoder, EncoderDeref};

#[derive(Debug)] 
pub struct Cache<T: Encoder> {
    inner: T,
    cache: UnsafeCell<Option<Vec<u8>>>,
}

impl<T: Encoder> !EncoderDeref for Cache<T> {}

impl<T: Encoder> From<T> for Cache<T> {
    fn from(value: T) -> Self {
        Cache {
            inner: value,
            cache: UnsafeCell::new(None),
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
    fn encode_to_buffer(&self, buf: &mut super::prelude::Buffer) -> Result<()> {
        let cache = unsafe { &mut *self.cache.get() };
        if let Some(cache) = cache {
            buf.write_all(&cache)?;
        } else {
            let data = T::encode(self)?.into_inner();
            buf.write_all(&data)?;
            *cache = Some(data);
        }
        Ok(())
    }
}
