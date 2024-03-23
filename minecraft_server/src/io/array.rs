use std::io::Result;
use std::{io::Read, io::Write};

use super::encoding::{Decoder, Encoder};
use super::prelude::Buffer;
use super::var_int::{VarIntRead, VarIntWrite};

pub trait VarIntSizedVecRead<D>: Sized {
    fn read_var_int_sized_vec(&mut self) -> Result<Vec<D>>;
}

pub trait VarIntSizedVecWrite<E> {
    fn write_var_int_sized_vec(&mut self, vec: &Vec<E>) -> Result<()>;
}

impl<E: Encoder> VarIntSizedVecWrite<E> for Buffer {
    fn write_var_int_sized_vec(&mut self, vec: &Vec<E>) -> Result<()> {
        self.write_var_i32(vec.len() as i32)?;
        for ele in vec {
            ele.encode_to_buffer(self)?;
        }
        Ok(())
    }
}

impl<D> VarIntSizedVecRead<D> for Buffer
where
    D: Decoder,
{
    fn read_var_int_sized_vec(&mut self) -> Result<Vec<D>> {
        let len = self.read_var_i32()?;
        let mut buf = Vec::<D>::with_capacity(len as usize);
        for _ in 0..len {
            let ele = D::decode_from_read(self)?;
            buf.push(ele);
        }
        Ok(buf)
    }
}
