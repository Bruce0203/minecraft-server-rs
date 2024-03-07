use std::io::Result;

use uuid::Uuid;

pub trait UuidRead {
    fn read_uuid(&mut self) -> Result<Uuid>;
}

pub trait UuidWrite {
    fn write_uuid(&mut self, value: Uuid) -> Result<()>;
}

impl<R: std::io::Read> UuidRead for R {
    fn read_uuid(&mut self) -> Result<Uuid> {
        let mut buf = [0; 16];
        self.read_exact(&mut buf)?;
        Ok(Uuid::from_bytes(buf))
    }
}

impl<W: std::io::Write> UuidWrite for W {
    fn write_uuid(&mut self, value: Uuid) -> Result<()> {
        self.write_all(&value.into_bytes())?;
        Ok(())
    }
}
