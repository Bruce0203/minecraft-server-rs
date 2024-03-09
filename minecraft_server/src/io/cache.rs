use std::{
    cell::UnsafeCell,
    io::{prelude::Write, Result},
    ops::Deref,
};

use super::prelude::Encoder;

pub struct Cache<T: Encoder> {
    inner: T,
    cache: UnsafeCell<Option<Vec<u8>>>,
}

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

