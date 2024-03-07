use std::io::{Error, ErrorKind, Read, Result, Write};
use std::{io::Cursor, net::SocketAddr};

use flate2::write::{ZlibDecoder, ZlibEncoder};
use flate2::Compression;
use mio::{net::TcpStream, Token};

use crate::io::prelude::{Encoder, VarIntRead, VarIntWrite};
use crate::protocol::v1_20_4::v1_20_4::V1_20_4;
use crate::server::prelude::Server;

use super::packet_writer::PacketIdentnifier;
use super::prelude::{PacketHandler, SessionRelay};

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
    pub fn handle_packet_read<const MAX_PACKET_BUFFER_SIZE: usize>(
        &mut self,
        server: &mut Server,
    ) -> Result<()> {
        self.fill_read_buf_from_socket_stream::<MAX_PACKET_BUFFER_SIZE>()?;
        self.read_packet_from_read_buf(server)?;
        let write_buf = &self.write_buf.get_ref()[..self.write_buf.position() as usize];
        self.stream.write_all(write_buf)?;
        self.write_buf.set_position(0);
        Ok(())
    }

    pub fn send_packet<E: Encoder + PacketIdentnifier>(&mut self, encoder: &E) -> Result<()> {
        let mut write_buf = self.encode_to_packet(encoder)?;
        self.handle_write_packet(&mut write_buf)
    }

    fn fill_read_buf_from_socket_stream<const MAX_PACKET_BUFFER_SIZE: usize>(
        &mut self,
    ) -> Result<()> {
        let mut pos = self.read_buf.position() as usize;
        let read_len = self.stream.read(&mut self.read_buf.get_mut()[pos..])?;
        pos += read_len;
        if read_len == 0 || pos >= MAX_PACKET_BUFFER_SIZE {
            Err(Error::new(ErrorKind::BrokenPipe, "BrokenPipe"))?
        }
        self.read_buf.set_position(pos as u64);
        Ok(())
    }

    fn read_packet_from_read_buf(&mut self, server: &mut Server) -> Result<()> {
        let read_len = self.read_buf.position();
        self.read_buf.set_position(0);
        let mut do_read = || -> Result<()> {
            while self.read_buf.position() != read_len {
                let packet_len = self.read_buf.read_var_i32()?;
                let pos = self.read_buf.position() as usize;
                self.packet_buf = Cursor::new(Vec::from(
                    &self.read_buf.get_ref()[pos..pos + packet_len as usize],
                ));
                self.read_buf.read_exact(self.packet_buf.get_mut())?;
                self.process_decompression()?;
                self.process_packet_read(server);
            }
            self.read_buf.set_position(0);
            Ok(())
        };
        if let Err(err) = do_read() {
            self.read_buf.set_position(read_len);
            return Err(err);
        }
        Ok(())
    }

    fn process_packet_read(&mut self, server: &mut Server) -> Result<()> {
        let player = self;
        match player.session_relay.protocol_id {
            0 => {
                PacketHandler::handle_packet(&V1_20_4, server, player)?;
            }
            765 => {
                PacketHandler::handle_packet(&V1_20_4, server, player)?;
            }
            n => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("unknown protocol: {:?}", n),
                ))
            }
        }
        Ok(())
    }

    fn process_decompression(&mut self) -> Result<()> {
        if self.session_relay.compression_threshold == -1 {
            Ok(())
        } else {
            let decompressed_len = self.packet_buf.read_var_i32()?;
            if self.session_relay.compression_threshold > decompressed_len {
                Ok(())
            } else {
                let mut d =
                    ZlibDecoder::new(Cursor::new(Vec::with_capacity(decompressed_len as usize)));
                d.write_all(self.packet_buf.get_ref())?;
                self.packet_buf = d.finish()?;
                Ok(())
            }
        }
    }

    fn encode_to_packet<E: Encoder + PacketIdentnifier>(
        &mut self,
        encoder: &E,
    ) -> Result<Cursor<Vec<u8>>> {
        let mut payload_buf = Cursor::new(Vec::new());
        let packet_id = encoder.get_packet_id(self)?;
        payload_buf.write_var_i32(packet_id)?;
        encoder.encode_to_write(&mut payload_buf)?;
        Ok(payload_buf)
    }

    fn handle_write_packet(&mut self, buf: &mut Cursor<Vec<u8>>) -> Result<()> {
        let compression_threshold = self.session_relay.compression_threshold;
        if compression_threshold != -1 {
            let packet_len = self.packet_buf.position() as i32;
            let packet_payload = &self.packet_buf.get_ref()[..packet_len as usize];
            if compression_threshold > packet_len {
                self.write_buf.write_var_i32(packet_len + 1)?;
                self.write_buf.write_all(&[0x00])?;
                self.write_buf.write_all(packet_payload)?;
            } else {
                let mut encoder = ZlibEncoder::new(Cursor::new(Vec::new()), Compression::default());
                encoder.write_all(packet_payload)?;
                let compressed_bytes = encoder.finish()?;
                let mut data = Cursor::new(Vec::new());
                data.write_var_i32(packet_len)?;
                data.write_all(compressed_bytes.get_ref())?;
                self.write_buf.write_var_i32(data.get_ref().len() as i32)?;
                self.write_buf.write_all(data.get_ref())?;
            }
        } else {
            self.write_buf.write_var_i32(buf.position() as i32)?;
            self.write_buf.write_all(buf.get_ref())?;
        }
        Ok(())
    }
}
