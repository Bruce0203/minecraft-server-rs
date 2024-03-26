use std::{
    io::{Error, ErrorKind},
    time::SystemTime,
};

use mio::Registry;

use crate::{
    io::prelude::Cache,
    net::prelude::{MaxPacketBufferSize, SelectorUpdateListener, SocketSelector},
};

use super::{
    chat::Chat,
    server_status::{Players, SamplePlayers, ServerStatus, ServerVersion},
};

pub struct GameServer {
    pub last_tick: SystemTime,
    pub start_time: SystemTime,
    pub server_status: Cache<ServerStatus>,
}

impl GameServer {
    pub fn new() -> GameServer {
        GameServer {
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
            }
            .into(),
            last_tick: SystemTime::UNIX_EPOCH,
            start_time: SystemTime::now(),
        }
    }
}

impl SelectorUpdateListener<GameServer> for GameServer {
    fn on_update(selector: &mut SocketSelector<GameServer>) {}

    fn on_init(selector: &mut SocketSelector<GameServer>) {}
}

impl MaxPacketBufferSize for GameServer {
    const MAX_PACKET_BUFFER_SIZE: usize = 100_000;
}
