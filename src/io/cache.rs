use std::{
    cell::{RefCell, UnsafeCell},
    io::{prelude::Write, Result},
    mem::MaybeUninit,
    ops::Deref,
};

use qcell::{LCell, LCellOwner};

use super::prelude::Encoder;

pub struct Cache<T: Encoder> {
    inner: T,
    cache: UnsafeCell<MaybeUninit<Vec<u8>>>,
    is_cached: UnsafeCell<bool>,
}

impl<T: Encoder> From<T> for Cache<T> {
    fn from(value: T) -> Self {
        Cache {
            inner: value,
            cache: UnsafeCell::new(MaybeUninit::uninit()),
            is_cached: UnsafeCell::new(false),
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
        LCellOwner::scope(|mut owner| {
            let mut is_cached = owner.rw(&self.is_cached);
            let cache = owner.rw(&self.cache);
            if *is_cached {
                writer
                    .write_all(unsafe { cache.assume_init_ref() })
                    .unwrap();
            } else {
                let data = T::encode(self)?.into_inner();
                writer.write_all(&data)?;
                self.cache.borrow_mut().write(data);
                *is_cached = true;
            }
        });
        Ok(())
    }
}
