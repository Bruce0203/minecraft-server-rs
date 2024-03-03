use std::{
    io::{Cursor, Error, ErrorKind},
    net::SocketAddr,
    ops::Deref,
};

use flate2::read;
use mc_io::var_int::{self, VarIntRead};
use socket_selector::ConnectionHandler;

use crate::{server::prelude::{Player, Server}, protocol::v1_20_4::V1_20_4};

use super::{packet_reader, prelude::{SessionRelay, PacketReadHandler}};

impl ConnectionHandler<Player> for Server {
    fn handle_connection_closed(&mut self, _socket: &mut Player) {}

    fn handle_connection_read(&mut self, socket: &mut Player, buf: &[u8]) -> std::io::Result<()> {
        let mut pos = 0;
        while pos < buf.len() {
            let buf = &buf[pos..];
            let (packet_len, read_len) = var_int::read_var_i32_fast(buf)?;
            let packet_len = packet_len as usize;
            pos += packet_len + read_len;
            let mut buf = &buf[read_len..packet_len + 1];
            let value =
                &mut packet_reader::read_packet_id_and_payload(buf, &mut socket.session_relay)?;
            match socket.session_relay.protocol_id {
                0 => {
                    V1_20_4::handle_packet_read(self, socket, value)?;
                }
                765 => {
                    V1_20_4::handle_packet_read(self, socket, value)?;
                }
                n => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!("unknown protocol: {:?}", n),
                    ))
                }
            }
        }
        Ok(())
    }

    fn handle_update(&mut self) {}

    fn handle_connection_accept(
        &mut self,
        stream: mio::net::TcpStream,
        token: mio::Token,
        addr: SocketAddr,
    ) -> Player {
        Player {
            stream,
            token,
            addr,
            session_relay: SessionRelay::default(),
        }
    }
}
