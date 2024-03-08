use std::io::Result;
use std::{io::Read, io::Write};

use super::encoding::{Decoder, Encoder};
use super::var_int::{VarIntWrite, VarIntRead};

pub trait VarIntSizedVecRead<D>: Sized {
    fn read_var_int_sized_vec(&mut self) -> Result<Vec<D>>;
}

pub trait VarIntSizedVecWrite {
    fn write_var_int_sized_vec<E: Encoder>(&mut self, vec: &Vec<E>) -> Result<()>;
}

impl<W> VarIntSizedVecWrite for W
where
    W: Write,
{
    fn write_var_int_sized_vec<E: Encoder>(&mut self, vec: &Vec<E>) -> Result<()> {
        self.write_var_i32(vec.len() as i32)?;
        for ele in vec {
            ele.encode_to_write(self)?;
        }
        Ok(())
    }
}

impl<D, R> VarIntSizedVecRead<D> for R
where
    R: Read,
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
