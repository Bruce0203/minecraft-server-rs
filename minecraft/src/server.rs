use uuid::Uuid;

use super::chat::Chat;

pub struct Server {
    version: i32,
    description: Chat,
    favicon: Option<String>,
    enforce_secure_chat: bool,
    previews_chat: bool,
}

pub struct Players {
    max: i32,
    online: i32,
    sample: Vec<SamplePlayer>,
}

pub struct SamplePlayer {
    name: String,
    id: Uuid,
}

impl Server {
    pub fn new() -> Server {
        Server {
            version: 764,
            description: Chat::from("A Minecraft Server"),
            favicon: None,
            enforce_secure_chat: false,
            previews_chat: false,
        }
    }
}
