use std::io::{Error, ErrorKind};

use crate::io::prelude::Cache;

use super::{
    chat::Chat,
    server_status::{Players, SamplePlayers, ServerStatus, ServerVersion},
};

pub struct Server {
    pub server_status: Cache<ServerStatus>,
}

impl Server {
    pub fn new() -> Server {
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
            }
            .into(),
        }
    }
}
