use std::{
    io::{Cursor, Error, Read, Result, Write},
    str::FromStr,
    vec,
};

use uuid::Uuid;

use crate::{
    io::prelude::{Buffer, Decoder, Encoder, ToIdentifier, VarInt},
    net::prelude::{PacketHandler, PacketWriter, Socket},
    protocol::v1_20_4::{
        login::login_play::LoginPlay,
        play::{
            change_difficulty::{Difficulty, S2CChangeDifficulty},
            keep_alive::{KeepAlive, KeepAlivePlayS2c},
            player_abilities::{PlayerAbilities, PlayerAbility},
            player_info::{InformedPlayer, PlayerInfoActions, PlayerInfoUpdate},
            set_center_chunk::SetCenterChunk,
            set_container_contents::SetContainerContent,
            set_container_slot::SetContainerSlot,
            set_default_position::SetDefaultPosition,
            set_entity_metadata::SetEntityMetadata,
            set_health::SetHealth,
            set_held_item::{S2CSetHeldItem, SetHeldItem},
            set_render_distance::SetRenderDistance,
            set_simulation_distance::SetSimulationDistance,
            synchronize_player_position::SyncPlayerPosition,
            system_chat_message::SystemChatMessage,
            update_attributes::{AttributeProperty, UpdateAttributes},
            update_light::UpdateLight,
            update_time::UpdateTime,
        },
    },
    server::{
        chunk::{Chunk, HeightMaps, LongArray},
        coordinates::{DoublePosition, FloatRotation, Location, Position},
        light::Light,
        metadata::prelude::{EntityMetadata, PlayerByte, PlayerMeta},
        prelude::{Chat, ConnectionState, EntityMeta, GameMode, GamePlayer, GameServer},
        slot::Slot,
    },
};

use super::server_data::ServerData;

pub struct FinishConfigurationC2s;
pub struct FinishConfigurationS2c;

impl Decoder for FinishConfigurationC2s {
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(FinishConfigurationC2s)
    }
}
impl Encoder for FinishConfigurationC2s {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        Ok(())
    }
}

impl Encoder for FinishConfigurationS2c {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        Ok(())
    }
}

impl Decoder for FinishConfigurationS2c {
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(FinishConfigurationS2c)
    }
}

impl PacketHandler<GameServer> for FinishConfigurationC2s {
    fn handle_packet(
        &self,
        server: &mut GameServer,
        player: &mut Socket<GamePlayer>,
    ) -> Result<()> {
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
        S2CSetHeldItem(SetHeldItem { slot: 0 }).send_packet(player)?;
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
        }
        .send_packet(player)?;
        SetRenderDistance { view_distance: 32 }.send_packet(player)?;
        SetSimulationDistance {
            simulation_distance: 32,
        }
        .send_packet(player)?;
        SetCenterChunk {
            chunk_x: 0,
            chunk_z: 0,
        }
        .send_packet(player)?;
        UpdateAttributes {
            entity_id: 0,
            properties: vec![AttributeProperty {
                key: "generic.movement_speed".to_identifier(),
                value: 0.1,
                modifiers: vec![],
            }],
        }
        .send_packet(player)?;
        UpdateTime {
            world_age: 0,
            time_of_day: 0,
        }
        .send_packet(player)?;
        SyncPlayerPosition {
            location: Location {
                pos: DoublePosition {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                rot: FloatRotation {
                    yaw: 0.0,
                    pitch: 0.0,
                },
            },
            flags: 0,
            teleport_id: 0,
        }
        .send_packet(player)?;
        SetContainerContent {
            window_id: 0,
            state_id: 1,
            slot_data: vec![Slot::None; 46],
            carried_item: Slot::None,
        }
        .send_packet(player)?;
        SetContainerSlot {
            window_id: 0,
            state_id: 2,
            slot: 45,
            slot_data: Slot::None,
        }
        .send_packet(player)?;
        let mut meta = PlayerMeta::default();
        //meta.living_entity.health = Some(20.0);
        //meta.player_byte = Some(PlayerByte::all());
        SetEntityMetadata {
            entity_id: 0,
            metadata: EntityMetadata(Box::new(meta)),
        }
        .send_packet(player)?;
        SetHealth {
            health: 20.0,
            food: 20,
            food_saturation: 5.0,
        }
        .send_packet(player)?;
        UpdateTime {
            world_age: 1000,
            time_of_day: 100,
        }
        .send_packet(player)?;
        S2CChangeDifficulty {
            new_difficulty: Difficulty::Easy,
            difficulty_locked: false,
        }
        .send_packet(player)?;
        let chunk_view_distance = 8;
        for x in -chunk_view_distance..chunk_view_distance {
            for z in -chunk_view_distance..chunk_view_distance {
                UpdateLight {
                    x,
                    z,
                    light: Light::new(),
                }
                .send_packet(player)?;
            }
        }
        for x in -chunk_view_distance..chunk_view_distance {
            for z in -chunk_view_distance..chunk_view_distance {
                Chunk::new(x, z).send_packet(player)?;
            }
        }
        SystemChatMessage {
            content: Chat::from("hello!".to_string()),
            overlay: false,
        }
        .send_packet(player)?;
        KeepAlivePlayS2c(KeepAlive { id: 0 }).send_packet(player);
        KeepAlivePlayS2c(KeepAlive { id: 1 }).send_packet(player);
        Ok(())
    }
}
