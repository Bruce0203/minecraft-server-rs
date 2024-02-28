use std::ops::Deref;

use json::JsonValue;
use uuid::Uuid;

use crate::chat::Chat;

pub struct ServerStatus {
    pub version: ServerVersion,
    pub description: Chat,
    pub favicon: Option<String>,
    pub enforce_secure_chat: bool,
    pub previews_chat: bool,
    pub players: Players,
}

#[derive(Debug)]
pub struct ServerVersion {
    pub name: String,
    pub protocol: i32,
}

#[derive(Debug)]
pub struct Players {
    pub max: i32,
    pub online: i32,
    pub sample: SamplePlayers,
}

#[derive(derive_more::Deref, Debug)]
pub struct SamplePlayers(pub Vec<SamplePlayer>);

impl SamplePlayers {
    pub fn new() -> SamplePlayers {
        SamplePlayers(Vec::new())
    }
}

#[derive(Debug)]
pub struct SamplePlayer {
    name: String,
    id: Uuid,
}

impl From<&ServerStatus> for JsonValue {
    fn from(value: &ServerStatus) -> Self {
        let mut data = json::JsonValue::new_object();
        data["version"] = (&value.version).into();
        data["players"] = (&value.players).into();
        let chat_data: JsonValue = Into::into(&value.description);
        data["description"] = chat_data;
        if let Some(favicon) = &value.favicon {
            data["favicon"] = favicon.as_str().into();
        }
        data["enforceSecureChat"] = value.enforce_secure_chat.into();
        data["previewsChat"] = value.previews_chat.into();
        data
    }
}

impl From<&ServerVersion> for JsonValue {
    fn from(value: &ServerVersion) -> Self {
        let mut data = json::JsonValue::new_object();
        data["name"] = value.name.as_str().into();
        data["protocol"] = value.protocol.into();
        data
    }
}

impl From<&Players> for JsonValue {
    fn from(value: &Players) -> Self {
        let mut data = json::JsonValue::new_object();
        data["max"] = value.max.into();
        data["online"] = value.online.into();
        data["sample"] = (&value.sample).into();
        data
    }
}

impl From<&SamplePlayers> for JsonValue {
    fn from(value: &SamplePlayers) -> Self {
        let mut data = json::JsonValue::new_array();
        for sample_player in value.iter() {
            let sample_player_data: JsonValue = sample_player.into();
            data.push(sample_player_data);
        }
        data
    }
}

impl From<&SamplePlayer> for JsonValue {
    fn from(value: &SamplePlayer) -> Self {
        let mut data = json::JsonValue::new_object();
        data["id"] = value.id.to_string().as_str().into();
        data["name"] = value.name.as_str().into();
        data
    }
}
