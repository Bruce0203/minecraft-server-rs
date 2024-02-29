pub mod server_status;
use std::{
    cell::RefCell,
    io::{Error, ErrorKind, Result},
    net::SocketAddr,
    ops::Deref,
    rc::Rc,
};

use bytes::{Buf, BytesMut};
use common_server::{
    selector::{self, ConnectionHandler, ConnectionPool},
    var_int::VarIntRead,
};
use mio::{net::TcpListener, Poll, Registry};
use uuid::Uuid;

use crate::{
    connection::{PacketReadHandler, SessionRelay},
    prelude::Player,
    protocol::v1_20_4::V1_20_4,
};

use self::server_status::{Players, SamplePlayers, ServerStatus, ServerVersion};

use super::chat::Chat;

pub struct Server {
    pub server_status: ServerStatus,
}

impl Server {
    pub fn new(addr: SocketAddr) -> Server {
        Server {
            server_status: ServerStatus {
                version: ServerVersion {
                    name: "1.20.4".to_string(),
                    protocol: 765,
                },
                description: Chat::from("A Minecraft Server".to_string()),
                favicon: None,
                enforce_secure_chat: true,
                previews_chat: true,
                players: Players {
                    max: 20,
                    online: 0,
                    sample: SamplePlayers::new(),
                },
            },
        }
    }
}

impl ConnectionHandler<Player> for Server {
    fn handle_connection_closed(&mut self, _socket: &mut Player) {}

    fn handle_connection_read(&mut self, socket: &mut Player, buf: &[u8]) -> Result<()> {
        let value = &mut BytesMut::from(buf);
        while !value.is_empty() {
            let packet_len = value.reader().read_var_i32()?;
            if value.len() < packet_len as usize {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "actual packet length is short than enough",
                ));
            }
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
