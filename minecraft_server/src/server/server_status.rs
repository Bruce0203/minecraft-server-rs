use std::io::{prelude::Write, Result};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::io::prelude::{Encoder, VarIntWrite, VarStringWrite};

use super::chat::Chat;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerStatus {
    pub version: ServerVersion,
    pub description: Chat,
    pub favicon: Option<String>,
    pub enforce_secure_chat: bool,
    pub previews_chat: bool,
    pub players: Players,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerVersion {
    pub name: String,
    pub protocol: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Players {
    pub max: i32,
    pub online: i32,
    pub sample: SamplePlayers,
}

#[derive(derive_more::Deref, Debug, Serialize, Deserialize)]
pub struct SamplePlayers(pub Vec<SamplePlayer>);

impl SamplePlayers {
    pub fn new() -> SamplePlayers {
        SamplePlayers(Vec::new())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlayer {
    name: String,
    id: Uuid,
}

impl Encoder for ServerStatus {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        let server_status_data = serde_json::to_string(&self)?;
        buf.write_var_string(server_status_data.as_str())?;
        Ok(())
    }
}
