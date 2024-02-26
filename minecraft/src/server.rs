use uuid::Uuid;

use super::chat::Chat;

pub struct Server {
    pub server_status: ServerStatus,
}

pub struct ServerStatus {
    pub version: i32,
    pub description: Chat,
    pub favicon: Option<String>,
    pub enforce_secure_chat: bool,
    pub previews_chat: bool,
}

pub struct Players {
    pub max: i32,
    pub online: i32,
    pub sample: Vec<SamplePlayer>,
}

pub struct SamplePlayer {
    name: String,
    id: Uuid,
}

impl Server {
    pub fn new() -> Server {
        Server {
            server_status: ServerStatus {
                version: 765,
                description: Chat::from("A Minecraft Server"),
                favicon: None,
                enforce_secure_chat: false,
                previews_chat: false,
            },
        }
    }
}
