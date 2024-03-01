pub mod packet_read_handler;
pub mod packet_reader;
pub mod packet_writer;
pub mod session_relay;

use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;
use std::net::SocketAddr;

use bytes::Buf;
use bytes::BytesMut;
use common_server::selector::ConnectionHandler;
use common_server::var_int::VarIntRead;
pub use packet_read_handler::PacketReadHandler;
pub use session_relay::ConnectionState;
pub use session_relay::SessionRelay;

use crate::prelude::Player;
use crate::protocol::v1_20_4::V1_20_4;
use crate::server::Server;

impl ConnectionHandler<Player> for Server {
    fn handle_connection_closed(&mut self, _socket: &mut Player) {}

    fn handle_connection_read(&mut self, socket: &mut Player, buf: &[u8]) -> Result<()> {
        let mut value = &mut packet_reader::read_packet_id_and_payload(buf, &mut socket.session_relay)?;
        while !value.is_empty() {
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
            write_buffer: BytesMut::new(),
        }
    }
}
