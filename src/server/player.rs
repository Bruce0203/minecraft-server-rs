use std::io::{Error, ErrorKind, Read, Result, Write};
use std::{io::Cursor, net::SocketAddr};

use flate2::write::ZlibDecoder;
use mio::{net::TcpStream, Token};

use crate::io::prelude::VarIntRead;
use crate::protocol::prelude::SessionRelay;

use super::prelude::Server;

pub struct Player {
    pub stream: TcpStream,
    pub token: Token,
    pub addr: SocketAddr,
    pub session_relay: SessionRelay,
    pub read_buf: Cursor<Vec<u8>>,
    pub write_buf: Cursor<Vec<u8>>,
    pub packet_buf: Cursor<Vec<u8>>,
}

impl Player {
    pub fn handle_packet_read<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(&mut Self) -> Result<()>,
    {
        let read_len = self.fill_read_buf_from_stream()? as u64;
        match self.read_packet(read_len, f) {
            Ok(_) => {}
            Err(err) => {
                println!("Err while read packet: {}", err);
                self.read_buf.set_position(read_len);
            }
        }
        self.stream
            .write_all(&self.write_buf.get_ref()[..self.write_buf.position() as usize])?;
        self.write_buf.set_position(0);
        Ok(())
    }

    pub fn fill_read_buf_from_stream(&mut self) -> Result<usize> {
        let read_len = self.stream.read(self.read_buf.get_mut())?;
        if read_len == 0 || self.read_buf.position() >= Server::MAX_PACKET_BUFFER_SIZE {
            Err(Error::new(ErrorKind::BrokenPipe, "BrokenPipe"))?
        }
        Ok(read_len)
    }

    pub fn read_packet<F>(&mut self, read_len: u64, f: F) -> Result<()>
    where
        F: Fn(&mut Player) -> Result<()>,
    {
        self.read_buf.set_position(0);
        while self.read_buf.position() != read_len {
            let packet_len = self.read_buf.read_var_i32()?;
            let pos = self.read_buf.position() as usize;
            self.packet_buf = Cursor::new(Vec::from(
                &self.read_buf.get_ref()[pos..pos + packet_len as usize],
            ));
            self.read_buf.read_exact(self.packet_buf.get_mut())?;
            //self.process_decompression()?;
            f(self)?;
        }
        Ok(())
    }

    pub fn process_decompression(&mut self) -> Result<()> {
        if self.session_relay.compression_threshold == -1 {
            return Ok(());
        } else {
            let decompressed_len = self.packet_buf.read_var_i32()?;
            if self.session_relay.compression_threshold > decompressed_len {
                Ok(())
            } else {
                let mut d =
                    ZlibDecoder::new(Cursor::new(Vec::with_capacity(decompressed_len as usize)));
                d.write_all(self.packet_buf.get_ref())?;
                let _ = d.finish()?;
                Ok(())
            }
        }
    }
}
