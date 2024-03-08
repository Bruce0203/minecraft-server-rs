use std::io::prelude::Write;

use crate::{
    io::prelude::{
        Encoder, I64Write, OptionWrite, U128Write, U8Write, VarIntSizedVecWrite, VarIntWrite,
        VarStringWrite, WriteBool,
    },
    net::prelude::{PacketIdentnifier, Player},
    server::{chat::ChatNbtWrite, prelude::GameMode},
};
use uuid::Uuid;

use crate::server::prelude::Chat;

pub struct PlayerInfoUpdate {
    pub players: Vec<InformedPlayer>,
}

pub struct InformedPlayer {
    pub uuid: Uuid,
    pub action: Vec<PlayerInfoActions>,
}

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

pub struct PlayerProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

pub struct PlayerChatSignature {
    chat_session_id: Uuid,
    public_key_expiry_time: i64,
    encoded_public_key: Vec<u8>,
    public_key_signature: Vec<u8>,
}

impl Encoder for PlayerChatSignature {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_u128(self.chat_session_id.as_u128())?;
        writer.write_i64(self.public_key_expiry_time)?;
        writer.write_var_i32(self.encoded_public_key.len() as i32)?;
        writer.write_all(&self.encoded_public_key)?;
        writer.write_var_i32(self.public_key_signature.len() as i32)?;
        writer.write_all(&self.public_key_signature)?;
        Ok(())
    }
}

impl Encoder for PlayerProperty {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_var_string(&self.name)?;
        writer.write_var_string(&self.value)?;
        if let Some(signature) = &self.signature {
            writer.write_bool(true)?;
            writer.write_var_string(&signature)?;
        } else {
            writer.write_bool(false)?;
        }
        Ok(())
    }
}

impl Encoder for PlayerInfoUpdate {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        if self.players.is_empty() {
            writer.write_all(&[0x00, 0x00])?;
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
        writer.write_u8(actions)?;
        writer.write_var_i32(self.players.len() as i32)?;
        for player in &self.players {
            writer.write_u128(player.uuid.as_u128())?;
            for action in &player.action {
                match action {
                    PlayerInfoActions::AddPlayer { name, properties } => {
                        writer.write_var_string(name.as_str())?;
                        writer.write_var_int_sized_vec(properties)?;
                    }
                    PlayerInfoActions::InitializeChat { signature } => {
                        writer.write_option(signature)?;
                    }
                    PlayerInfoActions::UpdateGameMode { game_mode } => {
                        game_mode.encode_to_write(writer)?;
                    }
                    PlayerInfoActions::UpdateListed { listed } => {
                        writer.write_bool(*listed)?;
                    }
                    PlayerInfoActions::UpdateLatency { ping } => {
                        writer.write_var_i32(*ping)?;
                    }
                    PlayerInfoActions::UpdateDisplayName { display_name } => {
                        if let Some(display_name) = display_name {
                            writer.write_bool(true)?;
                            writer.write_nbt_chat(display_name)?;
                        } else {
                            writer.write_bool(false)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl PacketIdentnifier for PlayerInfoUpdate {
    fn get_packet_id(&self, player: &mut Player) -> std::io::Result<i32> {
        Ok(0x3C)
    }
}
