use std::io::{Read, Result};

use super::prelude::{Decoder, Encoder};
use derive_more::{Deref, From, Into};

#[derive(Deref, From, Into)]
pub struct VarLong(i64);

pub trait VarLongRead {
    fn read_var_long(&mut self) -> Result<i64>;
}

pub trait VarLongWrite {
    fn write_var_long(&mut self, value: i64) -> Result<usize>;
}

impl<R: Read> VarLongRead for R {
    fn read_var_long(&mut self) -> Result<i64> {
        let mut buf = [0];
        let mut ans = 0;
        for i in 0..8 {
            self.read_exact(&mut buf)?;
            ans |= ((buf[0] & 0b0111_1111) as i64) << 7 * i;
            if buf[0] & 0b1000_0000 == 0 {
                break;
            }
        }
        Ok(ans)
    }
}

impl<W: std::io::Write> VarLongWrite for W {
    fn write_var_long(&mut self, mut value: i64) -> std::result::Result<usize, std::io::Error> {
        let mut buf = [0];
        let mut cnt = 0;
        while value != 0 {
            buf[0] = (value & 0b0111_1111) as u8;
            value = (value >> 7) & (i64::max_value() >> 6);
            if value != 0 {
                buf[0] |= 0b1000_0000;
            }
            cnt += self.write(&mut buf)?;
        }
        Ok(cnt)
    }
}
