use std::{
    io::{Cursor, Error, Result, Write},
    str::FromStr,
};

use uuid::Uuid;

use crate::io::{
    encoding::Encoder,
    identifier::ToIdentifier,
    nbt::{NbtNetworkRead, NbtNetworkWrite},
};

use crate::{
    protocol::prelude::{ConnectionState, PacketHandler, PacketWriter},
    protocol::v1_20_4::{
        login_play::LoginPlay,
        player_abilities::{PlayerAbilities, PlayerAbility},
        server_data::ServerData,
        set_default_position::SetDefaultPosition,
        set_held_item::SetHeldItem,
    },
    server::{self, prelude::*},
};

use super::player_info::{InformedPlayer, PlayerInfoActions, PlayerInfoUpdate, PlayerProperty};

pub struct FinishConfiguration {}

impl FinishConfiguration {
    pub fn new() -> FinishConfiguration {
        FinishConfiguration {}
    }
}

impl TryFrom<&mut Cursor<Vec<u8>>> for FinishConfiguration {
    type Error = Error;

    fn try_from(_value: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(FinishConfiguration {})
    }
}

impl PacketHandler for FinishConfiguration {
    fn handle_packet(&self, player: &mut Player) -> Result<()> {
        player.session_relay.connection_state = ConnectionState::Play;
        let login_play = LoginPlay {
            entity_id: 0,
            is_hardcore: false,
            dimension_names: vec!["minecraft:overworld".to_identifier()],
            max_players: 20,
            view_distance: 32,
            simulation_distance: 32,
            reduce_debug_info: false,
            enable_respawn_screen: true,
            do_limited_crafting: false,
            dimension_type: "minecraft:overworld".to_identifier(),
            dimension_name: "minecraft:overworld".to_identifier(),
            hashed_seed: 0,
            game_mode: GameMode::Creative,
            previous_game_mode: None,
            is_debug: false,
            is_flat: true,
            death_location: None,
            portal_cooldown: 0,
        };
        login_play.send_packet(player)?;
        let player_abilities = PlayerAbilities {
            flags: PlayerAbility::Flying,
            flying_speed: 0.5,
            field_of_view_modifier: 0.1,
        };
        player_abilities.send_packet(player)?;
        SetHeldItem { slot: 0 }.send_packet(player)?;
        SetDefaultPosition {
            location: Position::new(0, 0, 0),
            angle: 0.0,
        }
        .send_packet(player)?;
        let server_data = ServerData {
            message_of_the_day: Chat::from("MC Sv ...".to_string()),
            icon: None,
            enforce_secure_chat: true,
        };
        server_data.send_packet(player)?;
        PlayerInfoUpdate {
            players: vec![InformedPlayer {
                uuid: Uuid::from_str("053d384b-5b9f-47d7-a5da-6885c497ce7f").unwrap(),
                action: vec![
                    PlayerInfoActions::AddPlayer {
                        name: "JetBrainer".to_string(),
                        properties: vec![],
                    },
                    PlayerInfoActions::InitializeChat { signature: None },
                    PlayerInfoActions::UpdateGameMode {
                        game_mode: GameMode::Survival,
                    },
                    PlayerInfoActions::UpdateListed { listed: true },
                    PlayerInfoActions::UpdateLatency { ping: 0 },
                    PlayerInfoActions::UpdateDisplayName { display_name: None },
                ],
            }],
        }.send_packet(player)?;
        Ok(())
    }
}

impl PacketWriter for FinishConfiguration {
    fn get_packet_id(&self, _player: &mut Player) -> Result<i32> {
        Ok(0x02)
    }
}

impl Encoder for FinishConfiguration {
    fn encode_to_write<W: Write>(&self, _writer: &mut W) -> Result<()> {
        Ok(())
    }
}
