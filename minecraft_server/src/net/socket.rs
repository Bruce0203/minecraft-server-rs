use std::char::MAX;
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::ops::{Deref, DerefMut};
use std::{io::Cursor, net::SocketAddr};

use flate2::write::{ZlibDecoder, ZlibEncoder};
use flate2::Compression;
use mio::{net::TcpStream, Token};

use crate::io::prelude::{Decoder, Encoder, VarIntRead, VarIntWrite};
use crate::protocol::v1_20_4::v1_20_4::V1_20_4;
use crate::server::prelude::LoginServer;

use super::prelude::{PacketHandler, PacketIdentifier, SessionRelay};

pub struct Socket<Player> {
    pub stream: TcpStream,
    pub token: Token,
    pub addr: SocketAddr,
    pub session_relay: SessionRelay,
    pub player_data: Player,
    pub read_buf: Cursor<Vec<u8>>,
    pub write_buf: Cursor<Vec<u8>>,
    pub packet_buf: Cursor<Vec<u8>>,
}

impl<Player> Socket<Player> {
    pub fn new<const MAX_PACKET_BUFFER_SIZE: usize>(
        stream: TcpStream,
        token: Token,
        addr: SocketAddr,
        player_data: Player,
    ) -> Socket<Player> {
        Socket {
            stream,
            token,
            addr,
            player_data,
            session_relay: SessionRelay::default(),
            read_buf: Cursor::new(Vec::from([0; MAX_PACKET_BUFFER_SIZE])),
            write_buf: Cursor::new(Vec::from([0; MAX_PACKET_BUFFER_SIZE])),
            packet_buf: Cursor::new(vec![]),
        }
    }
    pub fn process_decompression(&mut self) -> Result<()> {
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

    pub fn encode_to_packet<E: Encoder + PacketIdentifier<Player>>(
        &mut self,
        encoder: &E,
    ) -> Result<Cursor<Vec<u8>>> {
        let mut payload_buf = Cursor::new(Vec::new());
        let packet_id = encoder.get_protocol_id(self)?;
        payload_buf.write_var_i32(packet_id)?;
        encoder.encode_to_write(&mut payload_buf)?;
        Ok(payload_buf)
    }

    pub fn handle_write_packet(&mut self, buf: &mut Cursor<Vec<u8>>) -> Result<()> {
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

    pub fn send_packet<E: Encoder + PacketIdentifier<Player>>(
        &mut self,
        encoder: &E,
    ) -> Result<()> {
        let mut write_buf = self.encode_to_packet(encoder)?;
        self.handle_write_packet(&mut write_buf)
    }
}
