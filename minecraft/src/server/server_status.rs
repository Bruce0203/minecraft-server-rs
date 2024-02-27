use std::ops::Deref;

use json::JsonValue;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::chat::Chat;

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerStatus {
    pub version: ServerVersion,
    pub description: Chat,
    pub favicon: Option<String>,
    pub enforce_secure_chat: bool,
    pub previews_chat: bool,
    pub players: Players,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerVersion {
    pub name: String,
    pub protocol: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Players {
    pub max: i32,
    pub online: i32,
    pub sample: SamplePlayers,
}

#[derive(derive_more::Deref, Serialize, Deserialize, Debug)]
pub struct SamplePlayers(pub Vec<SamplePlayer>);

impl SamplePlayers {
    pub fn new() -> SamplePlayers {
        SamplePlayers(Vec::new())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlayer {
    name: String,
    id: Uuid,
}

#[test]
fn test_json_ser() {
    let server_status = ServerStatus {
        version: ServerVersion {
            name: "1.20.4".to_string(),
            protocol: 765,
        },
        description: Chat::from("A Minecraft Server".to_string()),
        favicon: None,
        enforce_secure_chat: false,
        previews_chat: false,
        players: Players {
            max: 20,
            online: 0,
            sample: SamplePlayers::new(),
        },
    };
    let result = serde_json::to_string_pretty(&server_status).unwrap();
}
