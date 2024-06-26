use std::io::prelude::Write;

use crate::{
    io::prelude::{
        Decoder, Encoder, I64Write, OptionWrite, U128Write, U8Write, VarIntSizedVecWrite,
        VarIntWrite, VarStringWrite, WriteBool,
    },
    net::prelude::{PacketId, Socket},
    server::{
        chat::ChatNbtWrite,
        prelude::{GameMode, GamePlayer},
    },
};
use uuid::Uuid;

use crate::server::prelude::Chat;

#[derive(Debug)]
pub struct PlayerInfoUpdate {
    pub players: Vec<InformedPlayer>,
}

#[derive(Debug)]
pub struct InformedPlayer {
    pub uuid: Uuid,
    pub action: Vec<PlayerInfoActions>,
}

#[derive(Debug)]
pub enum PlayerInfoActions {
    AddPlayer {
        name: String,
        properties: Vec<PlayerProperty>,
    },
    InitializeChat {
        signature: Option<PlayerChatSignature>,
    },
    UpdateGameMode {
        game_mode: GameMode,
    },
    UpdateListed {
        listed: bool,
    },
    UpdateLatency {
        ping: i32,
    },
    UpdateDisplayName {
        display_name: Option<Chat>,
    },
}

#[derive(Debug)]
pub struct PlayerProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

#[derive(Debug)]
pub struct PlayerChatSignature {
    chat_session_id: Uuid,
    public_key_expiry_time: i64,
    encoded_public_key: Vec<u8>,
    public_key_signature: Vec<u8>,
}

impl Encoder for PlayerChatSignature {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> std::io::Result<()> {
        buf.write_u128(self.chat_session_id.as_u128())?;
        buf.write_i64(self.public_key_expiry_time)?;
        buf.write_var_i32(self.encoded_public_key.len() as i32)?;
        buf.write_all(&self.encoded_public_key)?;
        buf.write_var_i32(self.public_key_signature.len() as i32)?;
        buf.write_all(&self.public_key_signature)?;
        Ok(())
    }
}

impl Encoder for PlayerProperty {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> std::io::Result<()> {
        buf.write_var_string(&self.name)?;
        buf.write_var_string(&self.value)?;
        if let Some(signature) = &self.signature {
            buf.write_bool(true)?;
            buf.write_var_string(&signature)?;
        } else {
            buf.write_bool(false)?;
        }
        Ok(())
    }
}

impl Encoder for PlayerInfoUpdate {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> std::io::Result<()> {
        if self.players.is_empty() {
            buf.write_all(&[0x00, 0x00])?;
            return Ok(());
        }
        let mut actions = 0;
        for action in &self.players[0].action {
            match action {
                PlayerInfoActions::AddPlayer {
                    name: _,
                    properties: _,
                } => actions |= 0x01,
                PlayerInfoActions::InitializeChat { signature } => actions |= 0x02,
                PlayerInfoActions::UpdateGameMode { game_mode: _ } => actions |= 0x04,
                PlayerInfoActions::UpdateListed { listed: _ } => actions |= 0x08,
                PlayerInfoActions::UpdateLatency { ping: _ } => actions |= 0x10,
                PlayerInfoActions::UpdateDisplayName { display_name: _ } => actions |= 0x20,
            };
        }
        buf.write_u8(actions)?;
        buf.write_var_i32(self.players.len() as i32)?;
        for player in &self.players {
            buf.write_u128(player.uuid.as_u128())?;
            for action in &player.action {
                match action {
                    PlayerInfoActions::AddPlayer { name, properties } => {
                        buf.write_var_string(name.as_str())?;
                        buf.write_var_int_sized_vec(properties)?;
                    }
                    PlayerInfoActions::InitializeChat { signature } => {
                        buf.write_option(signature)?;
                    }
                    PlayerInfoActions::UpdateGameMode { game_mode } => {
                        game_mode.encode_to_buffer(buf)?;
                    }
                    PlayerInfoActions::UpdateListed { listed } => {
                        buf.write_bool(*listed)?;
                    }
                    PlayerInfoActions::UpdateLatency { ping } => {
                        buf.write_var_i32(*ping)?;
                    }
                    PlayerInfoActions::UpdateDisplayName { display_name } => {
                        if let Some(display_name) = display_name {
                            buf.write_bool(true)?;
                            buf.write_nbt_chat(display_name)?;
                        } else {
                            buf.write_bool(false)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl Decoder for PlayerInfoUpdate {
    fn decode_from_read(reader: &mut crate::io::prelude::Buffer) -> std::io::Result<Self> {
        //TODO wip
        Ok(PlayerInfoUpdate { players: vec![] })
    }
}
