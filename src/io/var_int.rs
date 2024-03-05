use std::io::{Error, ErrorKind, Result};

pub fn read_var_i32_fast(buf: &[u8]) -> Result<(i32, usize)> {
    let mut val = 0;
    for i in 0..5 {
        let byte = buf[i];
        val |= (byte as i32 & 0b01111111) << (i * 7);
        if byte & 0b10000000 == 0 {
            return Ok((val, i + 1));
        }
    }
    Err(Error::new(ErrorKind::InvalidInput, "VarInt is too large"))
}

pub trait VarIntRead {
    fn read_var_i32(&mut self) -> Result<i32>;
    fn read_var_i64(&mut self) -> Result<i64>;
}

pub trait VarIntWrite {
    fn write_var_i32(&mut self, value: i32) -> Result<usize>;
    fn write_var_i64(&mut self, value: i64) -> Result<usize>;
}

impl<R> VarIntRead for R
where
    R: std::io::Read,
{
    fn read_var_i32(&mut self) -> Result<i32> {
        let mut val = 0;
        let buf = &mut [0u8];
        for i in 0..5 {
            self.read_exact(buf)?;
            let byte = buf[0];
            val |= (byte as i32 & 0b01111111) << (i * 7);
            if byte & 0b10000000 == 0 {
                return Ok(val);
            }
        }
        Err(Error::new(ErrorKind::InvalidInput, "VarInt is too large"))
    }

    fn read_var_i64(&mut self) -> Result<i64> {
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

impl<W> VarIntWrite for W
where
    W: std::io::Write,
{
    fn write_var_i32(&mut self, value: i32) -> Result<usize> {
        let x = value as u64;
        let stage1 = (x & 0x000000000000007f)
            | ((x & 0x0000000000003f80) << 1)
            | ((x & 0x00000000001fc000) << 2)
            | ((x & 0x000000000fe00000) << 3)
            | ((x & 0x00000000f0000000) << 4);

        let leading = stage1.leading_zeros();

        let unused_bytes = (leading - 1) >> 3;
        let bytes_needed = 8 - unused_bytes;

        // set all but the last MSBs
        let msbs = 0x8080808080808080;
        let msbmask = 0xffffffffffffffff >> (((8 - bytes_needed + 1) << 3) - 1);

        let merged = stage1 | (msbs & msbmask);
        let bytes = merged.to_le_bytes();

        Ok(self.write(unsafe { bytes.get_unchecked(..bytes_needed as usize) })?)
    }

    fn write_var_i64(&mut self, mut value: i64) -> Result<usize> {
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

#[test]
fn test_var_i32() {
    fn test(first: Vec<u8>, second: i32) {
        let mut vec = Vec::new();
        vec.write_var_i32(second).unwrap();
        assert_eq!(first, vec);
        let read = std::io::Cursor::new(vec).read_var_i32().unwrap();
        assert_eq!(read, second);
    }
    test(vec![0x00], 0);
    test(vec![0x01], 1);
    test(vec![0x02], 2);
    test(vec![0x7f], 127);
    test(vec![0x80, 0x01], 128);
    test(vec![0xff, 0x01], 255);
    test(vec![0xdd, 0xc7, 0x01], 25565);
    test(vec![0xff, 0xff, 0x7f], 2097151);
    test(vec![0xff, 0xff, 0xff, 0xff, 0x07], 2147483647);
    test(vec![0xff, 0xff, 0xff, 0xff, 0x0f], -1);
    test(vec![0x80, 0x80, 0x80, 0x80, 0x08], -2147483648);
}

