use super::{server_status::{ServerStatus, ServerVersion, Players, SamplePlayers}, chat::Chat};

pub struct Server {
    pub server_status: ServerStatus,
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
            },
        }
    }
}
