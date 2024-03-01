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
